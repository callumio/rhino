use anyhow::{bail, Result};
use procinfo::pid::stat;
use rayon::iter::ParallelBridge;
use rayon::prelude::*;
use std::fs::DirEntry;
use std::io::Error;
use std::path::PathBuf;
use std::process::Command;
use std::{fs, i32, str};

pub fn run(cmd: &str, args: &[String]) {
    let buf: Vec<Vec<&str>> = args.par_iter().map(|x| x.split(' ').collect()).collect();
    let args_vec = buf.concat();
    let run_cmd = Command::new(cmd)
        .args(args_vec)
        .spawn()
        .expect("Error starting command");
}

pub fn search(program: &str, all: bool) -> Result<bool> {
    let find = |x: Result<DirEntry, Error>| {
        let path = match x.ok() {
            Some(y) => y.path(),
            None => return false,
        };
        get_process_valid(path, program).unwrap_or(false)
    };

    let processes = fs::read_dir("/proc/")?.par_bridge();
    let processes_exist: Vec<bool> = processes.map(find).collect();

    let all_reduce = |x: bool, xs: bool| x & xs;
    let any_reduce = |x: bool, xs: bool| x | xs;
    let reduce_processes = match all {
        true => all_reduce,
        false => any_reduce,
    };
    Ok(processes_exist
        .par_iter()
        .cloned()
        .reduce(|| all, reduce_processes))
}

pub fn get_process_valid(f: PathBuf, g: &str) -> Result<bool> {
    if f.is_dir() {
        let pid = match f
            .file_name()
            .and_then(|r| r.to_string_lossy().parse::<i32>().ok())
        {
            Some(p) => p,
            None => {
                bail!("Could not find pid for process")
            }
        };
        let name: String = stat(pid)?.command;
        Ok(name == g)
    } else {
        bail!("Process was not a folder");
    }
}
