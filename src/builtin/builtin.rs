use std::fmt::Debug;
use std::path::PathBuf;

pub trait BuiltIn: Debug {
  fn handler(&self, work_dir: &mut PathBuf, args: &Vec<String>) -> i8 {
    eprintln!("jash: command not implemented");

    1
  }
}
