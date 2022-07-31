mod args;
mod ps;

use ps::{search, run};
use args::RhinoArgs;

fn main() {
    let args: RhinoArgs = argh::from_env();
    // TODO remove unwrap once working
    let program_name: &str = &args.name.unwrap_or_else(|| args.cmd.clone());

    if !(args.invert ^ search(program_name)) {
        run(&args.cmd, &args.cmd_options);
    }
    else {
        println!("Search criteria was not met, exiting...");
    }
}
