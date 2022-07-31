mod args;
mod ps;

use ps::{search, run};
use args::RhinoArgs;

fn main() {
    let args: RhinoArgs = argh::from_env();
    let program_name: String = args.name.unwrap_or(args.cmd);
    if !(args.invert ^ search(&program_name)) {
        todo!("run the program");
    }
    else {
        println!("Search criteria was not met, exiting...");
    }
}
