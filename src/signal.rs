use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::RwLock;

use lazy_static::lazy_static;
use nix::sys::signal;

pub use nix::sys::signal::Signal;

lazy_static! {
  static ref signal_handler: RwLock<HashMap<i32, Vec<fn()>>> = RwLock::new(HashMap::new());
}

extern "C" fn handle_signal(sig: i32) {
  let Ok(handler) = signal_handler.read() else {
    signal_handler.clear_poison();
    return
  };
  let default = Vec::new();

  for handle in handler.get(&sig).unwrap_or(&default) {
    handle()
  }
}

pub fn init() {
  let sig_action = signal::SigAction::new(
    signal::SigHandler::Handler(handle_signal),
    signal::SaFlags::empty(),
    signal::SigSet::empty()
  );

  for sig in Signal::iterator() {
    unsafe {
      signal::sigaction(sig, &sig_action);
    }
  }
}

pub fn subscribe(sig: i32, handle: fn()) {
  let Ok(mut handler) = signal_handler.write() else {
    signal_handler.clear_poison();
    return
  };

  handler.entry(sig).or_insert(Vec::new()).push(handle);
}
