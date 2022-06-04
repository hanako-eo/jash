use crate::builtin::BuiltIn;
use crate::command_line::CommandLine;
use crate::env::vars;

#[derive(Debug)]
pub struct UnSet;

impl UnSet {
  pub fn new() -> Self {
    Self {}
  }
}

impl BuiltIn for UnSet {
  fn handler(&mut self, command_line: &CommandLine) -> i8 {
    for key in command_line.args() {
      vars::remove(key);
    }

    0
  }
}
