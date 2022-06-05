use std::collections::HashMap;
use std::process::{Command, Stdio};

use nix::sys::signal;
pub use nix::sys::signal::Signal;
use nix::sys::wait::{waitpid, WaitStatus};
use nix::unistd::Pid;

use crate::command_line::CommandLine;
use crate::env::vars;

#[derive(Debug, Clone)]
pub struct Process {
  id: u32,
  command_line: CommandLine,
  envs: HashMap<String, String>
}

impl Process {
  pub fn new(command_line: CommandLine) -> Self {
    Self {
      id: 0,
      command_line,
      envs: HashMap::new()
    }
  }

  pub fn spawn(
    &mut self,
    stdin: Option<Stdio>,
    stdout: Option<Stdio>,
    stderr: Option<Stdio>
  ) -> &mut Self {
    let child = Command::new(self.command_line.path().unwrap())
      .args(self.command_line.args())
      .env_clear()
      .envs(vars::all())
      .envs(self.envs.iter())
      .current_dir(vars::get("PWD"))
      .stdout(stdout.unwrap_or_else(|| Stdio::inherit()))
      .stdin(stdin.unwrap_or_else(|| Stdio::inherit()))
      .stderr(stderr.unwrap_or_else(|| Stdio::inherit()))
      .spawn();

    match child {
      Ok(c) => {
        self.id = c.id();
      },
      Err(_) => ()
    };

    self
  }

  pub fn wait(&self) -> u8 {
    match waitpid(Pid::from_raw(self.id as i32), None) {
      Ok(status) => match status {
        WaitStatus::Exited(_, code) => code as u8,
        WaitStatus::Signaled(_, sig, _) => 128 + sig as u8,
        _ => 127
      },
      Err(_) => 127
    }
  }

  pub fn signal(&self, s: Signal) -> &Self {
    if self.id != 0 {
      signal::kill(Pid::from_raw(self.id as i32), s).unwrap();
    }
    self
  }

  pub fn intr(&self) -> &Self {
    self.signal(Signal::SIGINT);
    self
  }

  pub fn kill(&self) -> &Self {
    self.signal(Signal::SIGKILL);
    self
  }

  pub fn id(&self) -> u32 {
    self.id
  }

  pub fn command(&self) -> &String {
    &self.command_line.line
  }

}

// fn p() {
//   use nix::unistd::Pid;
// use nix::sys::signal::{self, Signal};

//   // Spawn child process.
//   let mut child = std::process::Command::stdout();
//   /* build rest of command */
//   child.spawn().unwrap();

//   // Send SIGTERM to child process.
//   signal::kill(Pid::from_raw(child.id()), Signal::SIGTERM).unwrap();
// }
