use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::builtin::BuiltIn;
use crate::command_line::CommandLine;
use crate::process::Process;

#[derive(Debug)]
pub struct Jobs{
  processes: Rc<RefCell<HashMap<u32, Process>>>
}

impl Jobs {
  pub fn new(processes: Rc<RefCell<HashMap<u32, Process>>>) -> Self {
    Self {
      processes
    }
  }
}

impl BuiltIn for Jobs {
  fn handler(&mut self, command_line: &CommandLine) -> u8 {
    let processes = self.processes.borrow();
    for (i, (id, process)) in processes.iter().enumerate() {
      println!("[{}] {} {:<18} {}", i+1, id, "t", process.command())
    }

    0
  }
}
