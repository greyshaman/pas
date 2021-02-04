use iso_country::Country;

pub struct Contact {
  email: String,
  phone: String,
  address: String,
  city: String,
  country: Country,
  zip: String,
}

#[cfg(test)]
mod tests {
  use super::*;
  use iso_country::Country;

  #[test]
  fn contact_should_have_email_phone_address_zip_city_country_fields() {
    let c = Contact {
      email: String::from("aa@bb.cc"),
      phone: String::from("123456789"),
      address: String::from("Test address"),
      city: String::from("Testburg"),
      country: Country::RU,
      zip: String::from("123456"),
    };

    assert_eq!("aa@bb.cc", c.email);
    assert_eq!("123456789", c.phone);
    assert_eq!("Test address", c.address);
    assert_eq!("Testburg", c.city);
    assert_eq!(Country::RU, c.country);
    assert_eq!("123456", c.zip);
  }
}