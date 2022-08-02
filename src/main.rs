mod args;
mod ps;

use std::process;

use args::RhinoArgs;
use ps::{run, search};

fn main() {
    let args: RhinoArgs = argh::from_env();
    // TODO remove unwrap once working
    let program_name: &str = &args.name.unwrap_or_else(|| args.cmd.clone());
    let exists = match search(program_name, args.all) {
        Ok(r) => r,
        Err(e) => {
            eprintln!("{}", e);
            process::exit(1);
        }
    };

    if !(args.invert ^ exists) {
        run(&args.cmd, &args.cmd_options);
    } else {
        println!("Search criteria was not met, exiting...");
    }
}
