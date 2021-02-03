use std::time::SystemTime;

pub struct Diff<T> {
  data: T,
  version: usize,
  time: SystemTime,
}

impl<T> Diff <T> {
  pub fn new(content: T) -> Self {
    Diff {
      data: content,
      version: 0,
      time: SystemTime::now(),
    }
  }
}

pub struct VersionedContainer<T> {
  changes: Vec<Box<Diff<T>>>,
}

impl<T> VersionedContainer<T> {
  pub fn new(content: T) -> Self {
    VersionedContainer {
      changes: vec![Box::new(Diff::new(content))],
    }
  }

  fn last_diff(&self) -> Option<Diff<T>> {
    let boxed_diff = self.changes.iter().last();
    *boxed_diff
  }

  // returns last versioned value wrapped in Option
  pub fn value(&self) -> Option<T> {
    match self.changes.iter().last() {
      Some(v) => Some(v.data),
      None => None
    }
  }

  pub fn versions(&self) -> usize {
    self.changes.len()
  }

  pub fn changed_at(&self) -> SystemTime {
    match self.changes.iter().last() {
      Some(diff) => diff.time,
      None => SystemTime::UNIX_EPOCH,
    }
  }

  pub fn modify(&mut self, new_content: T) -> &Self {
    self.changes.push(Diff { content: new_content, version:  })
    self
  }
} 

#[cfg(test)]
mod tests {
  use super::*;

  struct Planet {
    name: String,
    radius: f64,
    mass: f64,
  }

  #[test]
  fn new_versioned_container_should_return_init_value() {
    let v_int = VersionedContainer::new(&1);

    assert_eq!(Some(1), v_int.value());
  }

  #[test]
  fn new_container_should_have_one_version() {
    let v_int = VersionedContainer::new(&10);

    assert_eq!(1, v_int.versions());
  }

  #[test]
  fn stored_value_should_have_creation_time_less_then_current() {
    let vc = VersionedContainer::new(&"test");
    let changed_at = vc.changed_at();

    assert!(SystemTime::now() > changed_at);
  }

  #[test]
  fn setter_should_add_versioned_changes() {
    let planet = Planet { name: String::from("Earth"), radius: 6.0f64, mass: 3e30 };
    let mut vp = VersionedContainer::new(planet);

    vp.modify( Planet { mass: 2e30, ..planet } );
  }
}
