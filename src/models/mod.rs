pub mod inner_names;
pub mod human_names;
pub mod product;
pub mod user;
pub mod client;

pub trait Named {
  fn name(&self) -> String;
  fn set_name(&mut self, value: &str) -> &Self;
  fn description(&self) -> String;
  fn set_description(&mut self, value: &str) -> &Self;
}

pub trait HumanNamed {
  fn first_name(&self) -> String;
  fn set_first_name(&mut self, value: &str) -> &Self;
  fn middle_name(&self) -> String;
  fn set_middle_name(&mut self, value: &str) -> &Self;
  fn last_name(&self) -> String;
  fn set_last_name(&mut self, value: &str) -> &Self;
  
  // LastName FirstName MiddleName
  fn full_name(&self) -> String {
    let mut name_tokens: Vec<String> = Vec::new();
    if self.last_name().len() > 0 {
      name_tokens.push(self.last_name());
    }
    if self.first_name().len() > 0 {
      name_tokens.push(self.first_name());
    }
    if self.middle_name().len() > 0 {
      name_tokens.push(self.middle_name());
    }
    name_tokens.join(" ")
  }

  // FirstName LastName
  fn brief_name(&self) -> String {
    let mut name_tokens: Vec<String> = Vec::new();
    if self.first_name().len() > 0 {
      name_tokens.push(self.first_name());
    }
    if self.last_name().len() > 0 {
      name_tokens.push(self.last_name());
    }
    name_tokens.join(" ")
  }

  fn to_capitalize(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
      None => String::new(),
      Some(f) => {
        if f.is_uppercase() {
          String::from(s)
        } else {
          f.to_uppercase().chain(chars).collect()
        }
      }
    }
  }
}
