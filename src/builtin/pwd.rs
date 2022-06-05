use crate::command_line::CommandLine;
use crate::env::vars;

use super::BuiltIn;

#[derive(Debug)]
pub struct PWD;

impl PWD {
  pub fn new() -> Self {
    Self {}
  }
}

impl BuiltIn for PWD {
  fn handler(&mut self, _: &CommandLine) -> u8 {
    println!("{}", vars::get("PWD"));

    0
  }
}
