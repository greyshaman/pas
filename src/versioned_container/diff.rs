use std::time::SystemTime;

pub struct Diff<T> {
  data: T,
  version: usize,
  time: SystemTime,
}
