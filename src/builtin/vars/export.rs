use crate::builtin::BuiltIn;
use crate::command_line::CommandLine;
use crate::env::vars;

#[derive(Debug)]
pub struct Export;

impl Export {
  pub fn new() -> Self {
    Self {}
  }
}

impl BuiltIn for Export {
  fn handler(&mut self, command_line: &CommandLine) -> u8 {
    for arg in command_line.args() {
      match arg.split_once("=") {
        Some((key, value)) => vars::set(key, value),
        None => ()
      };
    }

    0
  }
}
