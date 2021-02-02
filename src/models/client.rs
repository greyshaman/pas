use super::human_named::{HumanNamed, HumanName};

pub struct Client {
  human_name: HumanName,
}

impl HumanNamed for Client {
  fn first_name(&self) -> String {
    self.human_name.first_name()
  }

  fn set_first_name(&mut self, value: &str) -> &Client {
    self.human_name.set_first_name(value);
    self
  }

  fn middle_name(&self) -> String {
    self.human_name.middle_name()
  }

  fn set_middle_name(&mut self, value: &str) -> &Client {
    self.human_name.set_middle_name(value);
    self
  }

  fn last_name(&self) -> String {
    self.human_name.last_name()
  }

  fn set_last_name(&mut self, value: &str) -> &Client {
    self.human_name.set_last_name(value);
    self
  }
}

impl Client {
  pub fn new() -> Client {
    Client {
      human_name: HumanName::new(),
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn new_client_instance_should_have_empty_first_name() {
    let client = Client::new();

    assert_eq!("", client.first_name());
  }

  #[test]
  fn first_name_setter_should_change_client_instance() {
    assert_eq!("Test", Client::new().set_first_name("Test").first_name());
  }

  #[test]
  fn first_name_should_be_capitalized() {
    assert_eq!("Милан", Client::new().set_first_name("милан").first_name());
  }

  #[test]
  fn new_client_instance_should_have_empty_middle_name() {
    let client = Client::new();

    assert_eq!("", client.middle_name());
  }

  #[test]
  fn middle_name_setter_should_change_client_instance() {
    assert_eq!("Test", Client::new().set_middle_name("Test").middle_name());
  }

  #[test]
  fn middle_name_should_be_capitalized() {
    assert_eq!("Васильевна", Client::new().set_middle_name("васильевна").middle_name());
  }

  #[test]
  fn new_client_instance_should_have_empty_last_name() {
    let client = Client::new();

    assert_eq!("", client.last_name());
  }

  #[test]
  fn last_name_setter_should_change_client_instance() {
    assert_eq!("Test", Client::new().set_last_name("Test").last_name());
  }

  #[test]
  fn last_name_should_be_capitalized() {
    assert_eq!("Решетников", Client::new().set_last_name("решетников").last_name());
  }
}