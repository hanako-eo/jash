mod key;

use std::io::{Read, Stdin};
use std::path::{Path, PathBuf};
use std::process::exit;
use std::str::FromStr;

use nix::libc::{STDIN_FILENO, VMIN, VTIME};
use nix::sys::termios::{
  tcgetattr, tcsetattr, ControlFlags, InputFlags, LocalFlags, SetArg, Termios
};

use crate::env;

pub struct Input {
  original_config: Termios,
  config: Termios,
  stdin: Stdin,
  value: String
}

impl Input {
  pub fn new() -> Self {
    let config = tcgetattr(STDIN_FILENO).unwrap_or_else(|_| unsafe { core::mem::zeroed() });

    Self {
      original_config: config.clone(),
      config,
      stdin: std::io::stdin(),
      value: String::new()
    }
  }

  pub fn init(&mut self) {
    self.config.input_flags &= !InputFlags::IGNBRK;
    self.config.input_flags |= InputFlags::BRKINT;

    /* Ignore framing and parity errors in input. */
    self.config.input_flags |= InputFlags::IGNCR;
    self.config.input_flags |= InputFlags::IGNPAR;
    self.config.input_flags &= !InputFlags::PARMRK;

    /* Do not strip eighth bit on input. */
    self.config.input_flags &= !InputFlags::ISTRIP;

    /* Do not do newline translation on input. */
    self.config.input_flags &= !(InputFlags::INLCR | InputFlags::IGNCR | InputFlags::ICRNL);

    /* Use 8-bit characters. This too may affect standard streams,
     * but any sane C library can deal with 8-bit characters. */
    self.config.control_flags &= !ControlFlags::CSIZE;
    self.config.control_flags |= ControlFlags::CS8;

    /* Enable receiver. */
    self.config.control_flags |= ControlFlags::CREAD;

    /* Let INTR/QUIT/SUSP/DSUSP generate the corresponding signals. */
    self.config.local_flags |= LocalFlags::ISIG;

    /* Enable noncanonical mode.
     * This is the most important bit, as it disables line buffering etc. */
    self.config.local_flags &= !LocalFlags::ICANON;

    /* Disable echoing input characters. */
    self.config.local_flags &=
      !(LocalFlags::ECHO | LocalFlags::ECHOE | LocalFlags::ECHOK | LocalFlags::ECHONL);

    /* Disable implementation-defined input processing. */
    self.config.local_flags &= !LocalFlags::IEXTEN;

    /* To maintain best compatibility with normal behaviour of terminals,
     * we set TIME=0 and MAX=1 in noncanonical mode. This means that
     * read() will block until at least one byte is available. */
    self.config.control_chars[VTIME] = 0;
    self.config.control_chars[VMIN] = 1;

    tcsetattr(STDIN_FILENO, SetArg::TCSANOW, &self.config);
  }

  pub fn read(&mut self) {
    let mut buf = [0; 4];
    loop {
      let i = self.stdin.read(&mut buf);
      println!("{i:?} {:?}", buf);
      if buf[0] == b'q' {
        tcsetattr(STDIN_FILENO, SetArg::TCSANOW, &self.original_config);
        exit(0)
      }
    }
  }
}

impl Drop for Input {
  fn drop(&mut self) {
    tcsetattr(STDIN_FILENO, SetArg::TCSANOW, &self.original_config);
  }
}

#[macro_export]
macro_rules! input {
  () => {
    input!("");
  };
  ($($arg:tt)*) => {{
    use std::io::Write;

    let input = std::io::stdin();
    let mut output = std::io::stdout();

    let _ = output.write_all(format!($($arg)*).as_bytes());
    let _ = output.flush();

    let mut buffer = String::new();
    let _ = input.read_line(&mut buffer);
    buffer
  }};
}

pub fn which<P: ?Sized + AsRef<Path>>(program: &P) -> Option<String> {
  for mut folder in env::vars::get("PATH")
    .split(":")
    .map(|x| PathBuf::from_str(x).unwrap())
  {
    folder.push(program.clone());
    if folder.exists() && folder.is_file() {
      return Some(folder.to_str().unwrap().to_string())
    }
  }
  None
}

#[test]
fn which_correct_link() {
  assert_eq!(which("cat"), Some("/usr/bin/cat".to_string()));
  assert_eq!(which("ls"), Some("/usr/bin/ls".to_string()));
  assert_eq!(which("sh"), Some("/usr/bin/sh".to_string()));

  assert_eq!(which("cd"), None);
  assert_eq!(which("ll"), None);
  assert_eq!(which("fg"), None);
}
