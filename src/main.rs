pub mod utils;

use utils::{ls::ls, cd::cd, write_file::write_file, read_file::read_file};
use std::path::{PathBuf};
use std::env;
use std::io::{self, Write};

fn get_command() -> String {
    loop {
        print!("$ ");
        loop {
            match io::stdout().flush() {
                Ok(_) => break,
                Err(_) => (),
            }
        }
        let mut command = String::new();    
        match io::stdin().read_line(&mut command) {
            Ok(_) => return String::from(command.trim()),
            Err(_) => println!("Failed reading, try again"),
        };
    }   
}

fn get_options(command_options_and_targets: std::str::Split<'_, char>) -> Vec<char> {
    let mut options: Vec<char> = Vec::<char>::new();

    for arg in command_options_and_targets {
        let mut chars = arg.chars();
        if chars.next() != Some('-') {
            continue;
        }
        while let Some(option) = chars.next() {
            if options.contains(&option) == false {
                options.push(option);
            }
        }
    }
    return options;
}

fn get_taregts(command_options_and_targets: std::str::Split<'_, char>) -> Vec<String> {
    let mut targets: Vec<String> = Vec::<String>::new();

    for arg in command_options_and_targets {
        let mut chars = arg.chars();
        if chars.next() == Some('-') {
            continue;
        }
        targets.push(String::from(arg));
    }
    return targets;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut curr_path: PathBuf;

    if args.len() > 1 {
        // if a starting directory is defined, use it
        curr_path = PathBuf::from(&args[1]);
    }
    else {
        // if a starting directory is not defined, use current directory
        curr_path = match env::current_dir() {
            Ok(path) => path,
            Err(_) => {
                println!("Failed getting current directory");
                return;
            }
        };
    }
    if curr_path.exists() == false || curr_path.is_dir() == false {
        println!("not a valid directory: {:?}", curr_path);
    }

    loop {
        let command = get_command();

        if command.len() == 0 {
            println!("Enter a command. Enter 'help' to see commands");
            continue;
        }

        let command_parts:(&str, &str)  = match command.split_once(' ') {
            None => (&command, ""),
            Some(value) => value,
        };
    
        let options = get_options(String::from(command_parts.1).split(' '));
        let targets = get_taregts(String::from(command_parts.1).split(' '));

        match command_parts.0 {
            "ls" => ls(&curr_path, options),
            "cd" => curr_path = cd(curr_path, targets),
            "pwd" => println!("{:?}", curr_path),
            "write" => write_file(&curr_path, targets, options),
            "read" => read_file(&curr_path, targets),
            "help" => {
                println!("");
                println!("ls: show content of current directory\n  usage: ls (options)\n  -l => show additional information\n  -a => show hidden entries\n");
                println!("cd: change directory\n  usage: cd <relative path>\n  '..' goes to parent directory\n");
                println!("pwd: print working directory\n  usage: pwd\n");
                println!("write: write/replace a file with text you input line by line. Enter a line with just 'q' to quit\n  usage: write (options) <target file relative path>\n  -a => append to file instead\n");
                println!("read: read one or more files\n  usage: read <relative file path> (<relative file path> <relative file path> ...)\n");
                println!("help: show this list\n");
                println!("exit: exit the program\n");
            }
            "exit" => break,
            _ => {println!("Unknown command. Enter 'help' to see commands"); continue;},
        }
    }
    return;
}
