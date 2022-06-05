mod builtin;
mod command_line;
mod env;
mod io;
mod parser;
mod process;

use std::cell::RefCell;
use std::collections::HashMap;
use std::path::PathBuf;
use std::rc::Rc;

use builtin::processes::{Wait, Jobs};
use builtin::vars::{Export, UnSet};
use builtin::{BuiltIn, Exit, CD, PWD};
use command_line::{CommandLine, CommandModifier};
use env::{system, vars};
use process::Process;

#[derive(Debug)]
struct App {
  pub background_processes: Rc<RefCell<HashMap<u32, Process>>>,
  pub built_ins: HashMap<&'static str, Box<dyn BuiltIn>>
}

impl App {
  fn new() -> Self {
    vars::create("PS1", "\x1b[1;34m\\w\x1b[0m $\n> ");
    vars::set("$", std::process::id().to_string());

    let background_processes = Rc::new(RefCell::new(HashMap::new()));
    let mut app = Self {
      background_processes: background_processes.clone(),
      built_ins: HashMap::new()
    };

    app.built_ins.insert("cd", Box::new(CD::new()));
    app.built_ins.insert("exit", Box::new(Exit::new()));
    app.built_ins.insert("export", Box::new(Export::new()));
    app.built_ins.insert("pwd", Box::new(PWD::new()));
    app.built_ins.insert("unset", Box::new(UnSet::new()));
    
    app.built_ins.insert("jobs", Box::new(Jobs::new(background_processes.clone())));
    app.built_ins.insert("wait", Box::new(Wait::new(background_processes.clone())));
    
    app
  }

  fn ps1(&self) -> String {
    let home = vars::get("HOME");
    let cwd = vars::get("PWD");

    let pwd = if cwd.starts_with(&home) {
      cwd.replacen(&home, "~", 1)
    } else {
      cwd.clone()
    };

    let work_path = PathBuf::from(&cwd);
    let current_folder = match work_path.file_name() {
      Some(name) => name.to_str().unwrap(),
      None => "/"
    };

    vars::get("PS1")
      .replace(r"\h", &system::get_hostname())
      .replace(r"\u", &system::get_user())
      .replace(r"\y", &cwd)
      .replace(r"\Y", &current_folder)
      .replace(r"\w", &pwd)
      .replace(r"\W", if &pwd == "~" { &"~" } else { &current_folder })
  }

  fn execute(&mut self, command_line: &CommandLine) -> u8 {
    if command_line.is_empty() {
      return vars::get("?").parse().unwrap_or(0) as u8
    }

    match self.built_ins.get_mut(command_line.command().as_str()) {
      Some(built_in) => return built_in.handler(command_line),
      None => ()
    }

    if command_line.exist() {
      return self.exec_process(command_line)
    }

    eprintln!("{}: command not found", command_line.command());
    127
  }

  fn exec_process(&mut self, command_line: &CommandLine) -> u8 {
    let mut process = command_line.to_process();
    process.spawn(None, None, None);

    if let CommandModifier::Background(next) = command_line.modifier() {
      {
        let mut background_processes = self.background_processes.borrow_mut();
        background_processes.insert(process.id(), process);
      }
      match next {
        Some(next_command) => return self.execute(next_command.as_ref()),
        None => ()
      };
    } else {
      let code = process.wait();
      return match command_line.modifier() {
        CommandModifier::And(next_command) =>
          return if code == 0 {
            self.execute(next_command.as_ref())
          } else {
            code
          },
        CommandModifier::Or(next_command) =>
          return if code > 0 {
            self.execute(next_command.as_ref())
          } else {
            code
          },
        CommandModifier::Then(next_command) => self.execute(next_command.as_ref()),
        _ => code
      }
    }
    0
  }
}

fn main() {
  let mut app = App::new();

  loop {
    let value = input!("{}", app.ps1());
    let command_line = value.trim().to_string();
    if command_line.is_empty() {
      continue
    }

    let code = app.execute(&CommandLine::new(command_line));
    vars::set("?", code.to_string());
  }
}
