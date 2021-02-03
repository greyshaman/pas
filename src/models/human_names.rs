use super::HumanNamed;

pub(super) struct HumanNames {
  first_name: String,
  middle_name: String,
  last_name: String,
}

impl HumanNames {
  pub(super) fn new() -> HumanNames {
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