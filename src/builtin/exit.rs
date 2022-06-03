use std::process;

use crate::command_line::CommandLine;

use super::BuiltIn;

#[derive(Debug)]
pub struct Exit;

impl Exit {
  pub fn new() -> Self {
    Self {}
  }
}

impl BuiltIn for Exit {
  fn handler(&mut self, _: &CommandLine) -> i8 {
    process::exit(0)
  }
}
