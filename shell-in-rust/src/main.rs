use std::env;
use std::io::{stdin, stdout, Write};
use std::path::Path;
use std::process::{Child, Command, Stdio};

// read line , separate into commands (depends on | or >) each command with its args
//run command, connect with the next command

fn set_prompt() {
    print!(">");
    stdout().flush().unwrap();
}

fn get_command_handler(
    command: &str,
    stdin: Stdio,
    stdout: Stdio,
) -> std::result::Result<std::process::Child, std::io::Error> {
    let mut parts = command.trim().split_whitespace();
    let command = parts.next().unwrap();
    let args = parts;

    let handler = Command::new(command)
        .stdin(stdin)
        .stdout(stdout)
        .args(args)
        .spawn();
    return handler;
}

fn main() {
    loop {
        set_prompt();

        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        // command args output

        let mut commands = input.trim().split(" | ").peekable();
        let mut prev_command: Option<std::process::Child> = None;
        while let Some(command) = commands.next() {
            let stdin1 = if let Some(prev_command_val) = prev_command {
                Stdio::from(prev_command_val.stdout.unwrap())
            } else {
                Stdio::inherit()
            };

            let stdout = if commands.peek().is_some() {
                Stdio::piped()
            } else {
                Stdio::inherit()
            };

            let handler = get_command_handler(command, stdin1, stdout);
            match handler {
                Ok(child) => prev_command = Some(child),
                Err(e) => {
                    eprintln!("{}", e);
                    prev_command = None;
                }
            };
        }

        if let Some(mut final_command) = prev_command {
            // block until the final command has finished
            final_command.wait().unwrap();
        }
    }
}
