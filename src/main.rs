extern crate dirs;
use std::path::Path;
use std::{
    env,
    io::{stdin, stdout, Write},
    process::Command,
};
extern crate shellexpand;
// function for `cd` command
fn change_dir(args: std::str::Split<&str>) -> Result<i8, std::io::Error> {
    let home_dir = dirs::home_dir().unwrap();
    let new_dir = args.peekable().peek().map_or(home_dir, |x| {
        // Expand environment variables
        let temp: &str = &shellexpand::env(*x).unwrap().to_string();
        // Expand the tilde.
        let temp: &str = &shellexpand::tilde(temp).to_string();
        println!("temp path: {}", temp);
        Path::new(temp).to_path_buf()
    });
    let root = Path::new(&new_dir);
    if let Err(e) = env::set_current_dir(&root) {
        eprintln!("{}", e);
        return Err(e);
    }
    return Ok(0);
}
fn run_command(command: &str, args: std::str::Split<&str>) {
    let child = Command::new(command).args(args).spawn();
    // gracefully handle malformed user input
    match child {
        Ok(mut child) => {
            child.wait().unwrap();
        }
        Err(e) => eprintln!("No such command found, error: {}", e),
    };
}
fn main_loop() -> Result<u8, String> {
    let prompt = String::from(">");
    loop {
        print!("{} ", prompt);
        // flush so that it prints before taking input.
        // Stuck on this for 25 mins
        stdout().flush().unwrap();
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        let mut full_command = input.trim().split(" | ");

        let command = full_command.next().unwrap();
        let args: std::str::Split<&str> = full_command;
        match command {
            "cd" => {
                change_dir(args).unwrap();
            }
            "exit" => {
                return Ok(0);
            }
            command => {
                run_command(command, args);
            }
        }
    }
}
fn main() {
    main_loop().unwrap();
}
