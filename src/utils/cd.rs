use std::path::{PathBuf};

use super::join_paths::join_paths;

pub fn cd(current_path: PathBuf, target_relative_path: Vec<String>) -> PathBuf {
  let target = match target_relative_path.get(0) {
    None => {println!("Not a valid path"); return current_path;},
    Some(p) => p.to_string(),
  };

  let new_path = match join_paths(&current_path, &target, true) {
    Some(p) => p,
    None => return current_path,
  };

  if new_path.is_dir() {
    return new_path;
  }
  
  println!("Not a directory");
  return current_path;
}