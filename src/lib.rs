mod errors;
mod interval;
mod interval_limit;
mod limit_value;

pub use crate::errors::Error;
pub use crate::limit_value::LimitValue;
pub use crate::interval::Interval;

#[cfg(test)]
mod tests {
  #[test]
  fn it_works() {
    assert_eq!(2 + 2, 4);
  }
}
