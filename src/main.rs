extern crate dirs;
extern crate log;
use std::path::Path;
use std::process::{Child, Command, Stdio};
use std::str;
use std::{
    env,
    io::{stdin, stdout, Write},
};
extern crate shellexpand;
// function for `cd` command
fn change_dir(args: str::SplitWhitespace) -> Result<i8, std::io::Error> {
    let home_dir = dirs::home_dir().unwrap();
    let new_dir = args.peekable().peek().map_or(home_dir, |x| {
        // Expand environment variables
        let temp: &str = &shellexpand::env(*x).unwrap().to_string();
        // Expand the tilde.
        let temp: &str = &shellexpand::tilde(temp).to_string();
        // println!("temp path: {}", temp);
        Path::new(temp).to_path_buf()
    });
    let root = Path::new(&new_dir);
    if let Err(e) = env::set_current_dir(&root) {
        eprintln!("{}", e);
        return Err(e);
    }
    return Ok(0);
}
fn run_command(
    command: &str,
    args: str::SplitWhitespace,
    stdin: Stdio,
    stdout: Stdio,
) -> Option<Child> {
    let child = Command::new(command)
        .stdin(stdin)
        .stdout(stdout)
        .args(args)
        .spawn();
    let mut previous_command: Option<Child> = None;
    // gracefully handle malformed user input
    match child {
        Ok(output) => {
            // println!("Output: {:#?}", output.stdout);
            previous_command = Some(output);
        }
        Err(e) => {
            eprintln!("No such command found, error: {}", e);
            previous_command = None;
        }
    };
    return previous_command;
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
        let mut full_command = input.trim().split(" | ").peekable();
        let mut prev_cmd: Option<Child> = None;
        while let Some(command) = full_command.next() {
            let mut parts = command.trim().split_whitespace();

            let command = parts.next().unwrap();
            let args = parts;
            match command {
                "cd" => {
                    change_dir(args).unwrap();
                    prev_cmd = None;
                }
                "exit" => return Ok(0),

                command => {
                    // read input from the output of the previouis command
                    let stdin = prev_cmd.map_or(Stdio::inherit(), |output: Child| {
                        log::debug!("Output: {:#?}", output);
                        Stdio::from(output.stdout.unwrap())
                    });
                    let stdout = if full_command.peek().is_some() {
                        // if there is another command after this then send the output of this to
                        // the next command
                        Stdio::piped()
                    } else {
                        Stdio::inherit()
                    };
                    prev_cmd = run_command(command, args, stdin, stdout);
                }
            }
        }
        if let Some(mut final_command) = prev_cmd {
            // block until the final command has finished
            final_command.wait().unwrap();
        }
    }
}
fn main() {
    env_logger::init();
    main_loop().unwrap();
}
