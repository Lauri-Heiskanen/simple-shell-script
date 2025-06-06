use std::path::Path;
use std::fs::{self, DirEntry, Metadata};

fn format_time(seconds: u64) -> String {
    if seconds < 60 {
        return format!("{} second(s)", seconds)
    }
    else if seconds < 3600 {
        return format!("{} minute(s)", seconds / 60)
    }
    else if seconds < 86400 {
        return format!("{} hour(s)", seconds / 3600)
    }
    else {
        return format!("{} day(s)", seconds / 86400)
    }
}

fn print_dir_entry(entry: DirEntry, metadata: Metadata, additional_information: bool) {
  if metadata.is_dir() {
    println!("dir: {:?}", entry.file_name());
  }
  else if metadata.is_file() {
    println!("file: {:?}", entry.file_name());
  }
  else if metadata.is_symlink() {
    println!("sym_link: {:?}", entry.file_name());
  }
  else {
    println!("unknown: {:?}", entry.file_name());
  }
  if additional_information {
    println!("  size: {} bytes", metadata.len());
    if let Ok(value) = metadata.created() {
      println!("  created: {} ago", match value.elapsed() {
        Ok(elapsed) => format_time(elapsed.as_secs()),
        Err(_) => String::from("?"),
      });
    }
    if let Ok(value) = metadata.modified() {
      println!("  modified: {} ago", match value.elapsed() {
        Ok(elapsed) => format_time(elapsed.as_secs()),
        Err(_) => String::from("?"),
      });
    }
    println!("");
  }
}

fn ls_helper(path: &Path, additional_information: bool, show_hidden: bool) {
  let entries = match fs::read_dir(path) {
      Ok(entries) => entries,
      Err(_) => {
          println!("Failed reading directory.");
          return;
      }
  };

  let mut dirs: Vec<(DirEntry, Metadata)> = Vec::new();
  let mut files: Vec<(DirEntry, Metadata)> = Vec::new();
  let mut sym_links: Vec<(DirEntry, Metadata)> = Vec::new();
  let mut unknowns: Vec<(DirEntry, Metadata)> = Vec::new();

  for entry_result in entries {
    if let Ok(entry) = entry_result {
      if show_hidden || entry.file_name().to_str().unwrap().chars().next() != Some('.') {
        if let Ok(metadata) = entry.metadata() {
          if metadata.is_dir() {
            dirs.push((entry, metadata));
          }
          else if metadata.is_file() {
            files.push((entry, metadata));
          }
          else if metadata.is_symlink() {
            sym_links.push((entry, metadata));
          }
          else {
            unknowns.push((entry, metadata));
          }
        }
      }
    }
  }
    
  for dir in dirs {
    print_dir_entry(dir.0, dir.1, additional_information);
  }

  for file in files {
    print_dir_entry(file.0, file.1, additional_information);
  }

  for sym_link in sym_links {
    print_dir_entry(sym_link.0, sym_link.1, additional_information);
  }

  for unknown in unknowns {
    print_dir_entry(unknown.0, unknown.1, additional_information);
  }
}

pub fn ls(path: &Path, options: Vec<char>) {
  ls_helper(path, options.contains(&'l'), options.contains(&'a'));
}
