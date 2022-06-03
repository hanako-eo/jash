use std::fmt::Debug;

use crate::command_line::CommandLine;

pub trait BuiltIn: Debug {
  fn handler(&mut self, _command_line: &CommandLine) -> i8 {
    eprintln!("jash: command not implemented");

    1
  }
}
