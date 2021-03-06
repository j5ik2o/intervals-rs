use std::cmp::Ordering;
use std::fmt::{Display, Formatter, Debug};

use crate::Error;
use std::hash::{Hash, Hasher};

/// A structure that represents a limit value.
#[derive(Debug, Clone, Eq, Ord)]
pub enum LimitValue<T> {
  /// finite limit value
  Limit(T),
  /// infinite limit value
  Limitless,
}

impl<T: ToString> Hash for LimitValue<T> {
  fn hash<H: Hasher>(&self, state: &mut H) {
    match self {
      LimitValue::Limit(value) => {
        "Limit".hash(state);
        value.to_string().hash(state)
      }
      LimitValue::Limitless => {
        "Limitless".hash(state);
      }
    }
  }
}

impl<T: Default> Default for LimitValue<T> {
  fn default() -> Self {
    LimitValue::Limit(T::default())
  }
}

impl<T: PartialEq> PartialEq for LimitValue<T> {
  fn eq(&self, other: &Self) -> bool {
    match (self, other) {
      (LimitValue::Limitless, LimitValue::Limitless) => true,
      (LimitValue::Limit(value), LimitValue::Limit(other_value)) => value == other_value,
      _ => false,
    }
  }
}

impl<T: PartialOrd> PartialOrd for LimitValue<T> {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    match (self, other) {
      (LimitValue::Limitless, LimitValue::Limitless) => Some(Ordering::Equal),
      (LimitValue::Limit(_), LimitValue::Limitless) => Some(Ordering::Greater),
      (LimitValue::Limitless, LimitValue::Limit(_)) => Some(Ordering::Less),
      (LimitValue::Limit(value), LimitValue::Limit(other_value)) => value.partial_cmp(other_value),
    }
  }
}

impl<T> From<Option<T>> for LimitValue<T> {
  fn from(value: Option<T>) -> Self {
    match value {
      None => LimitValue::Limitless,
      Some(v) => LimitValue::Limit(v),
    }
  }
}

impl<T> LimitValue<T> {
  /// Verify if this limit is finite.
  pub fn is_limit(&self) -> bool {
    matches!(self, LimitValue::Limit(_))
  }

  /// Verify if this limit is infinite.
  pub fn is_limitless(&self) -> bool {
    matches!(self, LimitValue::Limitless)
  }

  /// Get the limit value.
  pub fn as_value(&self) -> Result<&T, Error> {
    match self {
      LimitValue::Limit(a) => Ok(a),
      LimitValue::Limitless => Err(Error::NotFoundError),
    }
  }

  /// Get the limit value.
  pub fn as_value_or<'a, TF>(&'a self, default: TF) -> &T
  where
    TF: Fn() -> &'a T,
  {
    match self {
      LimitValue::Limit(a) => a,
      LimitValue::Limitless => default(),
    }
  }
}

impl<T: Display> Display for LimitValue<T> {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    match self {
      LimitValue::Limit(a) => write!(f, "Limit({})", a),
      LimitValue::Limitless => write!(f, "Limitless"),
    }
  }
}
