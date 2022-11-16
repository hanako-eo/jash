pub enum Key {
  Ctrl(Box<Key>),
  Key(char),
  ArrayUp,
  ArrayDown,
  ArrayLeft,
  ArrayRight,
  Esc
}
