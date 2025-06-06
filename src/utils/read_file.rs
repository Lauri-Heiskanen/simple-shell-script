use std::{fs::File, path::PathBuf};
use std::io::{self, Read, Write};
use super::join_paths::join_paths;

pub fn read_file_helper(current_path: &PathBuf, target_relative_path: String) {
  

  let target_path = match join_paths(&current_path, &target_relative_path, true) {
    Some(p) => p,
    None => {println!("Not a valid path"); return;},
  };

  if target_path.is_file() == false {
    println!("Not a file");
    return;
  }

  let mut file = match File::open(target_path) {
    Ok(f) => f,
    Err(_) => {println!("Failed opening file"); return;}
  };

  let mut buf = String::new();
  match file.read_to_string(&mut buf) {
    Ok(n) => n,
    Err(_) => {println!("Failed reading file"); return;}
  };
  
  print!("{}", buf);
  loop {
      match io::stdout().flush() {
          Ok(_) => break,
          Err(_) => (),
      }
  }
  return;
}

pub fn read_file(current_path: &PathBuf, target_relative_path: Vec<String>) {
  if target_relative_path.len() == 0 {
    println!("Enter one or more files to read");
    return;
  }

  for target in target_relative_path {
    read_file_helper(current_path, target);
    println!("");
  }
}