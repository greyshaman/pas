pub trait Named {
  fn name(&self) -> String;
  fn set_name(&mut self, value: &str) -> &Self;
  fn description(&self) -> String;
  fn set_description(&mut self, value: &str) -> &Self;
}

pub(super) struct InnerName {
  name: String,
  description: String,
}

impl InnerName {
  pub(super) fn new() -> InnerName {
    InnerName {
      name: String::new(),
      description: String::new(),
    }
  }
}

impl Named for InnerName {
  fn name(&self) -> String {
    String::from(&self.name)
  }

  fn set_name(&mut self, value: &str) -> &InnerName {
    self.name = String::from(value);
    self
  }

  fn description(&self) -> String {
    String::from(&self.description)
  }

  fn set_description(&mut self, value: &str) -> &InnerName {
    self.description = String::from(value);
    self
  }
}