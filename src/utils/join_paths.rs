use std::path::{PathBuf};

pub fn join_paths(current_path: &PathBuf, target_relative_path: &String, must_exist: bool) -> Option<PathBuf> {

  let mut new_path = current_path.clone();

  for step in target_relative_path.split('\\') {
    if step == ".." {
      if let Some(parent) = new_path.parent() {
        new_path = parent.to_path_buf();
      }
      else {
        return None;
      }
    }
    else {
      new_path = new_path.join(PathBuf::from(step));
      if must_exist && new_path.exists() == false {
        return None;
      }
    }
  }
  return Some(new_path);
}