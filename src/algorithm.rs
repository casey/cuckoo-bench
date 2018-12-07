use crate::common::*;

use self::Algorithm::*;

use std::fmt::{self, Display, Formatter};

#[derive(Copy, Clone, Debug, Serialize)]
pub enum Algorithm {
  Cuckoo,
  Cuckatoo,
  Cuckaroo,
}

impl Algorithm {
  pub fn all() -> &'static [Algorithm] {
    &[Cuckoo, Cuckatoo, Cuckaroo]
  }

  pub fn name(self) -> &'static str {
    match self {
      Cuckoo => "cuckoo",
      Cuckatoo => "cuckatoo",
      Cuckaroo => "cuckaroo",
    }
  }
}

impl Display for Algorithm {
  fn fmt(&self, f: &mut Formatter) -> fmt::Result {
    write!(f, "{}", self.name())
  }
}
