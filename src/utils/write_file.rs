use std::{fs::File, path::PathBuf};
use std::io::{self, Write};
use super::join_paths::join_paths;
use super::read_file::read_file_helper;

fn write_file_helper(current_path: &PathBuf, target_relative_path: String, append: bool) {
  let target_path = match join_paths(&current_path, &target_relative_path, false) {
    Some(p) => p,
    None => {println!("Not a valid path"); return;},
  };

  if target_path.exists() && target_path.is_file() == false {
    println!("Not a file");
    return;
  }

  let mut file: File;
  println!("Write the lines you want to write to the file. Submitting 'q' quits writing");
  
  if append {
    read_file_helper(current_path, target_relative_path);
    file = match File::options().create(true).append(true).open(&target_path) {
      Ok(f) => f,
      Err(_) => {println!("Failed opening file"); return;}
    };
  }
  else {
    file = match File::create(&target_path) {
      Ok(f) => f,
      Err(_) => {println!("Failed opening file"); return;}
    };
  }

  
  loop {
    let mut line = String::new();    
    match io::stdin().read_line(&mut line) {
        Ok(_) => (),
        Err(_) => println!("Failed reading, try again"),
    };
    if line.trim() == "q" {
      break;
    }
    match file.write_all(&line.into_bytes()) {
      Ok(_) => (),
      Err(_) => {println!("Failed writing file"); return;}
    };
  }

  println!("File written successfully");
  return;
}


pub fn write_file(current_path: &PathBuf, target_relative_path: Vec<String>, options: Vec<char>) {
  let target = match target_relative_path.get(0) {
    None => {println!("Not a valid path"); return;},
    Some(p) => p.to_string(),
  };

  write_file_helper(current_path, target, options.contains(&'a'));
}