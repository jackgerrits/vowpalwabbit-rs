use crossbeam_channel::bounded;
use std::cell::UnsafeCell;

use std::fs::File;
use std::io::{self, BufRead};

use vowpalwabbit::pool::ExamplePool;
use vowpalwabbit::workspace::Workspace;

fn main() {
    let args: Vec<String> = vec!["--cb_adf".to_owned(), "--quiet".to_owned()];
    let pool = ExamplePool::new();
    let workspace: UnsafeCell<Workspace> = Workspace::new(&args).unwrap().into();
    let (sender, r) = bounded(256);

    std::thread::scope(|s| unsafe {
        let ws_ref = workspace.get().as_ref().unwrap();
        s.spawn(|| {
            let file = File::open("example_datafile.json").unwrap();
            io::BufReader::new(file).lines().for_each(|line| {
                let line = line.unwrap();
                sender
                    .send(ws_ref.parse_decision_service_json(&line, &pool))
                    .unwrap();
            });

            // Done reading.
            std::mem::drop(sender);
        });

        loop {
            let res = r.recv();
            match res {
                Ok(ex) => {
                    let ws = workspace.get().as_mut().unwrap();
                    let mut e = ex.unwrap();
                    ws.setup_multi_ex(&mut e).unwrap();
                    ws.learn_multi_example(&mut e).unwrap();

                    pool.return_multi_example(e);
                }
                Err(_) => break,
            }
        }
    });
}
