use std::{
    io::{stdin, stdout, Write},
    process::Command,
};

fn main() {
    let prompt = String::from(">");
    // flush so that it prints before taking input.
    // Stuck on this for 25 mins
    print!("{}", (prompt + " "));
    stdout().flush().unwrap();
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let command = input.trim();
    println!("Command Recieved: {} ", command);
    Command::new(command).spawn().unwrap();
}
