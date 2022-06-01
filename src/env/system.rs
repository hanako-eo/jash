use std::ffi::OsString;

pub fn get_hostname() -> String {
  let hn = hostname::get().unwrap();
  os_to_string(hn)
}

pub fn get_user() -> String {
  let un = users::get_current_username().unwrap();
  os_to_string(un)
}

fn os_to_string(a: OsString) -> String {
  a.to_string_lossy().to_string()
}
