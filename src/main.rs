extern crate core;

use std::fs;
use std::fs::{ReadDir};
use std::process::{Command, exit, Output};
use crate::config::Config;

mod config;

const SAMPLE_CONFIG: &[u8] = include_bytes!("../sample_config.toml");

#[cfg(windows)]
const SPLIT_CHAR: char = '\\';
#[cfg(not(windows))]
const SPLIT_CHAR: char = '/';

fn main() {
    let config_bytes = if let Ok(config) = fs::read("./config.toml") {
        config
    } else {
        let file = SAMPLE_CONFIG.to_vec();
        fs::write("config.toml", file).unwrap();
        eprintln!("A default configuration was created as 'config.toml'");
        exit(1);
    };
    let config: Config = toml::from_slice(&config_bytes).expect("Cannot serialize toml");

    // Step 1: Copy over the WT cache into local cache
    for dir_entry in fs::read_dir(&config.wt_base_path).unwrap() {
        if let Ok(entry) = dir_entry {
            if let Ok(meta) = entry.metadata() {
                if meta.is_file() {
                   let file = fs::read(entry.path()).unwrap();
                    let path = format!("{}/{}", &config.target_path, entry.path().to_str().unwrap().split(SPLIT_CHAR).last().unwrap());
                    fs::write(&path,file).unwrap();
                }
            }
        }
    }


    // Step 2: Extract binaries into BLK format
    let mut handles = vec![];
    for read_dir in fs::read_dir(&config.target_path).unwrap() {
        for item in read_dir {
            if let Ok(meta) = item.metadata() {
                if meta.is_file()  {
                    if let Some(file_name) = item.file_name().to_str() {
                        let file_name = file_name.to_owned();
                        let config = config.clone();
                        let handle = std::thread::spawn(move ||{
                            run_vromfs_extract(config, file_name);
                        });
                        handles.push(handle);
                    }
                }
            }
        }
    }
    for handle in handles {
        handle.join().unwrap();
    }

    // Step 3: Extract .blk to .blkx
    let mut handles = vec![];
    for read_dir in fs::read_dir("cache/output").unwrap() {
        for item in read_dir {
            if let Ok(meta) = item.metadata() {
                if meta.is_dir()  {
                    if let Some(folder_name) = item.file_name().to_str() {
                        let folder_name = folder_name.to_owned();
                        let config = config.clone();
                        let handle = std::thread::spawn(move ||{
                            run_blk_extract(config, folder_name);
                        });
                        handles.push(handle);
                    }
                }
            }
        }
    }
    for handle in handles {
        handle.join().unwrap();
    }

    // Step 4: Delete excess files from output path
    delete_excess(fs::read_dir("cache/output").unwrap());
}

fn delete_excess(folder: ReadDir) {
    for file in folder {
        if let Ok(entry) = file {
            if let Ok(metadata) = entry.metadata() {
                if metadata.is_file()  {
                    if entry.file_name().to_str().unwrap().split('.').last().unwrap() == "blk" {
                        fs::remove_file(entry.path()).unwrap();
                    }
                } else {
                    delete_excess(fs::read_dir(entry.path()).unwrap());
                }
            }
        }
    }
}

fn run_vromfs_extract(config: Config, file_name: String) {
    let command = format!("cd {} && python -m wt_tools.vromfs_unpacker {}/{} -O {}/output", &config.wt_tools_path, &config.target_path, file_name, &config.target_path);
    let output = exec_command(&command);
    if !output.status.success() {
        panic!("{}", String::from_utf8(output.stderr).expect("Cannot build string from failed python run"));
    }
}

fn run_blk_extract(config: Config, extract_folder: String) {
    let command = format!("cd {} && python -m wt_tools.blk_unpack_ng --format=json {}/output/{extract_folder}", &config.wt_tools_path, &config.target_path);
    let output = exec_command(&command);
    if !output.status.success() {
        panic!("{}", String::from_utf8(output.stderr).expect("Cannot build string from failed python run"));
    }
}

fn exec_command(command: &str) -> Output {
   if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(["/C", &command])
            .output()
            .expect("failed to execute process")
    } else {
        Command::new("sh")
            .arg("-c")
            .arg(command)
            .output()
            .expect("failed to execute process")
    }
}
