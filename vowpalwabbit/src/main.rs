use vowpalwabbit;

use std::error::Error;

// Just an example to ensure that things worked so far.
fn main() -> Result<(), Box<dyn Error>> {
    let args = std::env::args().skip(1).collect::<Vec<String>>();
    let mut workspace = vowpalwabbit::Workspace::new(&args)?;
    workspace.run_driver()?;

    Ok(())
}
