mod args;
mod env;
mod io;
mod parser;

use std::path::PathBuf;
use std::process::Command;

use args::Args;
use env::Variables;

fn main() {
  let mut vars = Variables::new();
  let mut pwd_path = PathBuf::from(vars.get("PWD".to_string()).unwrap_or(&"/".to_string()));

  vars.set("PS1".to_string(), "\x1b[1;34m\\w\x1b[0m $\n> ".to_string());

  let hn = hostname::get().unwrap();
  let hn = hn.to_str().unwrap_or("pc");

  let username = users::get_current_username().unwrap();
  let username = username.to_str().unwrap();

  let home = env::get("HOME");

  loop {
    let full_pwd = pwd_path.to_str().unwrap();
    let pwd = if full_pwd.starts_with(&home) {
      full_pwd.replacen(&home, "~", 1)
    } else {
      full_pwd.to_string()
    };

    let current_folder = match pwd_path.file_name() {
      Some(name) => name.to_str().unwrap(),
      None => "/"
    };
    let ps1 = env::get("PS1")
      .replace(r"\u", username)
      .replace(r"\h", hn)
      .replace(r"\y", &full_pwd)
      .replace(r"\Y", &current_folder)
      .replace(r"\w", &pwd)
      .replace(r"\W", if &pwd == "~" {
        &"~"
      } else {
        &current_folder
      });

    let value = input!("{}", ps1);
    let command = value.trim().to_string();
    if command.is_empty() {
      continue
    }

    let program = Args::from_str(command);
    if let Some(path) = program.path {
      let r = Command::new(path)
        .args(program.args)
        .env_clear()
        .envs(vars.gets())
        .current_dir(full_pwd)
        .spawn();

      if let Ok(mut process) = r {
        vars.set("?".to_string(), match process.wait() {
          Ok(status) => status.code().unwrap_or(1).to_string(),
          Err(_) => "1".to_string()
        });
      }
    } else if program.program == "cd" {
      if let Some(dest) = program.args.get(0) {
        if dest.starts_with("/") {
          pwd_path.clear();
        }
        pwd_path.push(dest);
      } else {
        pwd_path.clear();
        pwd_path.push(&home);
      }
      pwd_path = pwd_path.canonicalize().unwrap();
    }
  }
}
