/* 
use std::env;
use std::fs;

use std::process::Command;

fn main() {
    // --snip--
    let file_path = String::from("path-to-app.txt");


    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    println!("content is:\n{contents}");


    let output = if cfg!(target_os = "windows") {
        Command::new("cmd")
                .args(["start", "Starting Qalculate", contents.as_str()])
                .output()
                .expect("failed to execute process")
    } else {
        Command::new("sh")
                .arg("-c")
                .arg("echo You're not on Windows")
                .output()
                .expect("failed to execute process")
    };

    let hello = output.stdout;
    //println!("{hello}");
}*/
use std::env;
use std::fs;
use std::process::Command;

use execute::Execute;

fn main() {
    let file_path = String::from("path-to-app.txt");
    let APP_PATH = fs::read_to_string(file_path)
    .expect("Should have been able to read the file");

    let APP_PATH_STR: &str = APP_PATH.as_str();
    let mut command = Command::new(APP_PATH_STR);

    //first_command.arg("-version");

    // if first_command.execute_check_exit_status_code(0).is_err() {
    //     eprintln!("The path `{}` is not a correct FFmpeg executable binary file.", FFMPEG_PATH);
    // }
        
    if let Some(exit_code) = command.execute().unwrap() {
        if exit_code == 0 {
            println!("Ok.");
        } else {
            eprintln!("Failed.");
        }
    } else {
        eprintln!("Interrupted!");
    }
}