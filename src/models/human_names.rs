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

pub(super) struct HumanNames {
  first_name: String,
  middle_name: String,
  last_name: String,
}

impl HumanNames {
  pub(super) fn new() -> Self {
    HumanNames {
      first_name: String::new(),
      middle_name: String::new(),
      last_name: String::new(),
    }
  }
}

impl HumanNamed for HumanNames {
  fn first_name(&self) -> String {
    HumanNames::to_capitalize(&self.first_name)
  }

  fn set_first_name(&mut self, value: &str) -> &HumanNames {
    self.first_name = String::from(value);
    self
  }
  
  fn middle_name(&self) -> String {
    HumanNames::to_capitalize(&self.middle_name)
  }

  fn set_middle_name(&mut self, value: &str) -> &HumanNames {
    self.middle_name = String::from(value);
    self
  }

  fn last_name(&self) -> String {
    HumanNames::to_capitalize(&self.last_name)
  }

  fn set_last_name(&mut self, value: &str) -> &HumanNames {
    self.last_name = String::from(value);
    self
  }
}