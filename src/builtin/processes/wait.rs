use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::builtin::BuiltIn;
use crate::command_line::CommandLine;
use crate::process::Process;

#[derive(Debug)]
pub struct Wait{
  processes: Rc<RefCell<HashMap<u32, Process>>>
}

impl Wait {
  pub fn new(processes: Rc<RefCell<HashMap<u32, Process>>>) -> Self {
    Self {
      processes
    }
  }
}

impl BuiltIn for Wait {
  fn handler(&mut self, command_line: &CommandLine) -> u8 {
    let processes = self.processes.borrow();
    if let Some(id) = command_line.args().get(0){
      let id: u32 = id.parse().unwrap();

      return match processes.get(&id) {
        Some(process) => process.wait(),
        None => 127
      }
    } 

    0
  }
}
