use std::process::Command;
use std::{i32, str, fs};
use procinfo::pid::stat;

pub fn run(cmd: &str, args: &str) {
    let args_vec: Vec<&str> = args.split(' ').collect();
    let run_cmd = Command::new(cmd).args(args_vec).spawn().expect("Error starting command");
}

pub fn search(program: &str) -> bool {
    for x in fs::read_dir("/proc/").unwrap() {
        let file = x.unwrap().path();
        if file.is_dir() {
            // TODO remove unwrap after testing
            let process_id: i32 = file.file_name().unwrap().to_string_lossy().parse::<i32>().unwrap_or(1);
            if stat(process_id).unwrap_or_default().command == program {
                return true;
            }
        };
    };

    false
}
