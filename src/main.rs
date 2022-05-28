mod args;
mod env;
mod io;
mod parser;

use std::path::PathBuf;

use args::Args;
use env::Variables;

fn main() {
  let mut vars = Variables::new();
  let pwd_path = PathBuf::from(vars.get("PWD".to_string()).unwrap_or(&"/".to_string()));

  vars.set("PS1".to_string(), "\x1b[1;34m\\w\x1b[0m $\n> ".to_string());

  let hn = hostname::get().unwrap();
  let hn = hn.to_str().unwrap_or("pc");

  let username = users::get_current_username().unwrap();
  let username = username.to_str().unwrap();

  let home = env::get("HOME");

  loop {
    let pwd = pwd_path.to_str().unwrap();
    let pwd = if pwd.starts_with(&home) {
      pwd.replacen(&home, "~", 1)
    } else {
      pwd.to_string()
    };

    let current_folder = pwd_path.file_name().unwrap().to_str().unwrap();
    let ps1 = env::get("PS1")
      .replace(r"\u", username)
      .replace(r"\h", hn)
      .replace(r"\w", &pwd)
      .replace(r"\W", current_folder);

    let value = input!("{}", ps1);
    let command = value.trim().to_string();
    if command.is_empty() {
      continue
    }

    let program = Args::from_str(command);
    println!("{:?}", program);
  }
}
