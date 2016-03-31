extern crate ardite;

#[macro_use]
mod read;

use ardite::Value;
use ardite::driver::Driver;

pub trait Tests {
  fn test_driver(name: &str, values: Vec<Value>) -> Box<Driver>;
}

#[macro_export]
macro_rules! test_driver {
  ($tests:path) => {
    test_driver_read!($tests);
  }
}
