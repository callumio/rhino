mod args;
mod ps;

use ps::{search, run};
use args::RhinoArgs;

fn main() {
    let args: RhinoArgs = argh::from_env();
    let program_name: String = args.name.unwrap_or(args.cmd);
}
