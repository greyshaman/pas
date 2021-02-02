use super::human_named::HumanNamed;

pub struct User {
  first_name: String,
  middle_name: String,
  last_name: String,
  user_name: String,
  encrypted_password: String,
}

impl HumanNamed for User {
  fn first_name(&self) -> String {
    User::to_capitalize(&self.first_name)
  }

  fn set_first_name(&mut self, value: &str) -> &User {
    self.first_name = String::from(value);
    self
  }
  
  fn middle_name(&self) -> String {
    User::to_capitalize(&self.middle_name)
  }

  fn set_middle_name(&mut self, value: &str) -> &User {
    self.middle_name = String::from(value);
    self
  }

  fn last_name(&self) -> String {
    User::to_capitalize(&self.last_name)
  }

  fn set_last_name(&mut self, value: &str) -> &User {
    self.last_name = String::from(value);
    self
  }
}

impl User {
  pub fn new() -> User {
    User {
      first_name: String::new(),
      middle_name: String::new(),
      last_name: String::new(),
      user_name: String::new(),
      encrypted_password: String::new(),
    }
  }

  pub fn user_name(&self) -> String {
    String::from(&self.user_name)
  }

  pub fn set_user_name(&mut self, value: &str) -> &User {
    self.user_name = String::from(value);
    self
  }

  pub fn encrypted_password(&self) -> String {
    String::from(&self.encrypted_password)
  }

  pub fn set_encrypted_password(&mut self, value: & str) -> &User {
    self.encrypted_password = String::from(value);
    self
  }
}


#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn new_user_instance_should_have_empty_first_name() {
    let user = User::new();

    assert_eq!("", user.first_name());
  }

  #[test]
  fn first_name_setter_should_change_user_instance() {
    assert_eq!("First", User::new().set_first_name("First").first_name());
  }

  #[test]
  fn first_name_should_be_capitalized() {
    assert_eq!("Милан", User::new().set_first_name("милан").first_name());
  }

  #[test]
  fn new_user_instance_should_have_empty_middle_name() {
    let user = User::new();

    assert_eq!("", user.middle_name());
  }

  #[test]
  fn middle_name_setter_should_change_user_instance() {
    assert_eq!("Middle", User::new().set_middle_name("Middle").middle_name());
  }

  #[test]
  fn middle_name_should_be_capitalized() {
    assert_eq!("Васильевна", User::new().set_middle_name("васильевна").middle_name());
  }

  #[test]
  fn new_user_instance_should_have_empty_last_name() {
    let user = User::new();

    assert_eq!("", user.last_name());
  }

  #[test]
  fn last_name_setter_should_change_user_instance() {
    assert_eq!("Last", User::new().set_last_name("Last").last_name());
  }

  #[test]
  fn last_name_should_be_capitalized() {
    assert_eq!("Решетников", User::new().set_last_name("решетников").last_name());
  }

  #[test]
  fn new_user_instance_should_have_empty_user_name() {
    let user = User::new();

    assert_eq!("", user.user_name());
  }

  #[test]
  fn user_name_setter_should_change_user_instance() {
    assert_eq!("testuser", User::new().set_user_name("testuser").user_name());
  }

  #[test]
  fn new_user_instance_should_have_empty_encrypted_password() {
    let user = User::new();

    assert_eq!("", user.encrypted_password());
  }

  #[test]
  fn encrypted_password_setter_should_change_user_instance() {
    assert_eq!("test_password", User::new().set_encrypted_password("test_password").encrypted_password());
  }
}