use std::path::PathBuf;
use std::process;

use crate::env::vars;

use super::BuiltIn;

#[derive(Debug)]
pub struct Exit;

impl Exit {
  pub fn new() -> Self {
    Self {}
  }
}

impl BuiltIn for Exit {
  fn handler(&self, work_dir: &mut PathBuf, args: &Vec<String>) -> i8 {
    process::exit(0)
  }
}
