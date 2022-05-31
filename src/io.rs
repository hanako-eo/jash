use std::path::{Path, PathBuf};
use std::str::FromStr;

use crate::env;

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
