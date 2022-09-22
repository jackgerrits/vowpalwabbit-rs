use std::cell::UnsafeCell;

use std::collections::BTreeSet;
use std::fs::{self, File};
use std::io::{self, BufRead};
use std::sync::Arc;

use anyhow::{anyhow, Context};
use std::path::PathBuf;
use std::vec;

use clap::{CommandFactory, Parser, Subcommand};
use rayon::prelude::*;

use anyhow::Result;
use vowpalwabbit::pool::{ExamplePool, ReturnToPool};
use vowpalwabbit::workspace::{Learn, RecordStats, SetupExample, Workspace};

#[derive(clap::ValueEnum, Clone, Debug)]
enum InputFormat {
    Dsjson,
}

#[derive(Parser)]
#[clap(version)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Train a VW model file. Currently only single pass and DSJSON input format is supported.
    Train(Train),
}

#[derive(Parser, Debug)]
struct Train {
    #[clap(
        short,
        long,
        parse(from_os_str),
        help = "List of input files to process"
    )]
    input: Vec<PathBuf>,

    #[clap(long, value_enum, default_value_t = InputFormat::Dsjson, help="Input format to interpret input files as")]
    input_format: InputFormat,

    #[clap(
        short,
        long,
        parse(from_os_str),
        help = "If provided, writes the final trained model to this file"
    )]
    output_model: Option<PathBuf>,

    #[clap(
        long,
        parse(from_os_str),
        help = "If provided, writes the final trained model as a readable model to this file. This is the same format as VW's --readable_model ..."
    )]
    readable_model: Option<PathBuf>,

    #[clap(
        long,
        help = "VW arguments to use for model training. Some arguments are not permitted as they are for driver configuration in VW or managed by this tool. For example you cannot supply --data yourself."
    )]
    model_args: Option<String>,

    #[clap(
        long,
        default_value_t = 32,
        help = "How many input lines to parse in each batch."
    )]
    batch_size: usize,

    #[clap(
        long,
        default_value_t = 512,
        help = "How many examples can be waiting to learned from before waiting."
    )]
    queue_size: usize,

    #[clap(
        long,
        default_value_t = 0,
        help = "Number of threads to use for parsing. 0 means select automatically."
    )]
    parse_threads: usize,
}

fn process_command_line(input: Option<String>) -> Result<Vec<String>> {
    let mut vw_args = match input {
        Some(value) => shlex::Shlex::new(&value).collect(),
        None => Vec::new(),
    };

    let args_set = BTreeSet::<String>::from_iter(vw_args.iter().cloned());
    let mut disallowed_args_set = BTreeSet::new();
    disallowed_args_set.insert("--quiet".to_string());
    disallowed_args_set.insert("--audit".to_string());
    disallowed_args_set.insert("-a".to_string());
    disallowed_args_set.insert("--version".to_string());
    disallowed_args_set.insert("--dry_run".to_string());
    disallowed_args_set.insert("-P".to_string());
    disallowed_args_set.insert("--progress".to_string());
    disallowed_args_set.insert("-h".to_string());
    disallowed_args_set.insert("--help".to_string());
    disallowed_args_set.insert("--onethread".to_string());
    disallowed_args_set.insert("--testonly".to_string());
    disallowed_args_set.insert("-t".to_string());
    disallowed_args_set.insert("--holdout_off".to_string());
    disallowed_args_set.insert("--holdout_period".to_string());
    disallowed_args_set.insert("--holdout_after".to_string());
    disallowed_args_set.insert("--early_terminate".to_string());
    disallowed_args_set.insert("--passes".to_string());
    disallowed_args_set.insert("--examples".to_string());
    disallowed_args_set.insert("--sort_features".to_string());
    disallowed_args_set.insert("-d".to_string());
    disallowed_args_set.insert("--data".to_string());
    disallowed_args_set.insert("--daemon".to_string());
    disallowed_args_set.insert("--foreground".to_string());
    disallowed_args_set.insert("--port".to_string());
    disallowed_args_set.insert("--num_children".to_string());
    disallowed_args_set.insert("--pid_file".to_string());
    disallowed_args_set.insert("--port_file".to_string());
    disallowed_args_set.insert("-c".to_string());
    disallowed_args_set.insert("--cache".to_string());
    disallowed_args_set.insert("--cache_file".to_string());
    disallowed_args_set.insert("--json".to_string());
    disallowed_args_set.insert("--dsjson".to_string());
    disallowed_args_set.insert("-k".to_string());
    disallowed_args_set.insert("--kill_cache".to_string());
    disallowed_args_set.insert("--compressed".to_string());
    disallowed_args_set.insert("--no_stdin".to_string());
    disallowed_args_set.insert("--no_daemon".to_string());
    disallowed_args_set.insert("--flatbuffer".to_string());
    disallowed_args_set.insert("--csv".to_string());
    disallowed_args_set.insert("--quiet".to_string());
    disallowed_args_set.insert("--driver_output_off".to_string());
    disallowed_args_set.insert("--driver_output".to_string());
    disallowed_args_set.insert("--log_level".to_string());
    disallowed_args_set.insert("--log_output".to_string());
    disallowed_args_set.insert("--limit_output".to_string());
    disallowed_args_set.insert("-f".to_string());
    disallowed_args_set.insert("--final_regressor".to_string());
    disallowed_args_set.insert("--readable_model".to_string());
    disallowed_args_set.insert("--invert_hash".to_string());
    disallowed_args_set.insert("--dump_json_weights_experimental".to_string());
    disallowed_args_set
        .insert("--dump_json_weights_include_feature_names_experimental".to_string());
    disallowed_args_set
        .insert("--dump_json_weights_include_extra_online_state_experimental".to_string());
    disallowed_args_set.insert("--save_per_pass".to_string());
    disallowed_args_set.insert("--output_feature_regularizer_binary".to_string());
    disallowed_args_set.insert("--output_feature_regularizer_text".to_string());
    disallowed_args_set.insert("--span_server".to_string());
    disallowed_args_set.insert("--unique_id".to_string());
    disallowed_args_set.insert("--total".to_string());
    disallowed_args_set.insert("--node".to_string());
    disallowed_args_set.insert("--span_server_port".to_string());
    disallowed_args_set.insert("--ring_size".to_string());
    disallowed_args_set.insert("--example_queue_limit".to_string());
    // We enforce this to true.
    disallowed_args_set.insert("--strict_parse".to_string());
    disallowed_args_set.insert("-p".to_string());
    disallowed_args_set.insert("--predictions".to_string());
    disallowed_args_set.insert("-r".to_string());
    disallowed_args_set.insert("--raw_predictions".to_string());
    // Todo allow to be controlled.
    disallowed_args_set.insert("--random_seed".to_string());
    disallowed_args_set.insert("--feature_mask".to_string());
    disallowed_args_set.insert("-i".to_string());
    disallowed_args_set.insert("--initial_regressor".to_string());
    disallowed_args_set.insert("--input_feature_regularizer".to_string());
    disallowed_args_set.insert("--sendto".to_string());

    for supplied_arg in &args_set {
        // This is not perfect. But let's start with it.
        if !supplied_arg.starts_with("--") && !supplied_arg.starts_with('-') {
            continue;
        }
        let opt = supplied_arg.split_terminator('=').next().unwrap();
        for illegal_arg in &disallowed_args_set {
            if opt.starts_with(illegal_arg) {
                return Err(anyhow!("{} is not a permitted option for model_args", opt));
            }
        }
    }

    vw_args.push("--quiet".to_owned());
    vw_args.push("--ring_size=1".to_owned());

    Ok(vw_args)
}

pub struct UnsafeWorkspaceWrapper {
    pub workspace: UnsafeCell<Workspace>,
}

impl UnsafeWorkspaceWrapper
{
    pub fn as_ref(&self) -> &Workspace
    {
        unsafe { self.workspace.get().as_ref().unwrap() }
    }

    pub fn as_mut(&self) -> &mut Workspace
    {
        unsafe { self.workspace.get().as_mut().unwrap() }
    }
}

unsafe impl Send for UnsafeWorkspaceWrapper {}
unsafe impl Sync for UnsafeWorkspaceWrapper {}

fn train(args: Train) -> Result<()> {
    rayon::ThreadPoolBuilder::new()
        .num_threads(args.parse_threads)
        .build_global()?;

    let vw_args = process_command_line(args.model_args)?;

    let pool = ExamplePool::new();

    let unsafe_workspace_cell: UnsafeCell<Workspace> = Workspace::new(&vw_args)
        .with_context(|| format!("Failed to create workspace with args {:?}", vw_args))?
        .into();
    // We use an unsafe cell, because parse_decision_service_json, and the learning code does not interact.
    let shareable_workspace: UnsafeWorkspaceWrapper = UnsafeWorkspaceWrapper {
        workspace: unsafe_workspace_cell,
    };
    let (tx, rx) = flume::bounded(args.queue_size);

    std::thread::scope(|s| -> Result<()> {
        s.spawn(|| {
            for file in args.input {
                let file = File::open(file).expect("Failed to open file");
                let mut line_iter = io::BufReader::new(file).lines();
                loop {
                    let mut batch = vec![];
                    for _ in 0..args.batch_size {
                        if let Some(line) = line_iter.next() {
                            batch.push(line.unwrap());
                        } else {
                            break;
                        }
                    }
                    if batch.is_empty() {
                        break;
                    }
                    let output_lines: Vec<_> = batch
                        .into_par_iter()
                        .map(|line| {
                                shareable_workspace.as_ref()
                                .parse_decision_service_json(&line, &pool)
                        })
                        .collect();

                    for line in output_lines {
                        tx.send(line).expect(
                            "Receiver should not be disconnected before all lines have been sent.",
                        );
                    }
                }
            }

            std::mem::drop(tx);
        });

        let unsafe_workspace_ref = shareable_workspace.as_mut();

        loop {
            // TODO consider skipping broken examples.
            let res = rx.recv();
            match res {
                Ok(line) => {
                    let mut ex = unsafe_workspace_ref.setup(line?)?;
                    unsafe_workspace_ref.learn(&mut ex)?;
                    unsafe_workspace_ref.record_stats(&mut ex)?;
                    pool.return_example(ex);
                }
                // Sender has been dropped. Stop here.
                Err(_) => break,
            }
        }
        unsafe_workspace_ref.end_pass()?;
        Ok(())
    })?;

    let unsafe_workspace_ref = shareable_workspace.as_ref();
    if let Some(model_file) = args.output_model {
        fs::write(model_file, &*unsafe_workspace_ref.serialize_model()?)?;
    }

    if let Some(model_file) = args.readable_model {
        fs::write(model_file, unsafe_workspace_ref.serialize_readable_model()?)?;
    }

    Ok(())
}

fn main() -> Result<()> {
    let args = Cli::parse();
    match args.command {
        Commands::Train(args) => {
            if args.input.is_empty() {
                let mut app = Cli::into_app();
                let sub = app
                    .find_subcommand_mut("train")
                    .expect("train must exist as a subcommand");
                sub.print_help()?;
                return Err(anyhow!("At least 1 input file is required."));
                // return;
            }
            train(args)
        }
    }
}
