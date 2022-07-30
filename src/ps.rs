use std::process::Command;
use std::str;

pub fn run(cmd: &str, args: &str) {
    let args_vec: Vec<&str> = args.split(' ').collect();
    let run_cmd = Command::new(cmd).args(args_vec).spawn().expect("Error starting command");
}

pub fn search(program: &str) -> bool {
    let search_cmd = Command::new("pgrep").args(["-ix", program]).output().unwrap();
    !matches!(search_cmd.stdout.as_slice(), b"")
}
