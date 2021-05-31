use std::cmp::Ordering;
use std::fmt::{Display, Formatter, Debug};

use crate::LimitValue;
use std::hash::Hash;

#[derive(Debug, Clone, Hash, Eq, Ord)]
pub struct IntervalLimit<T: Display + Clone + Hash + Ord> {
  closed: bool,
  lower: bool,
  value: LimitValue<T>,
}

impl<T: Display + Clone + Hash + Eq + Ord + PartialEq + PartialOrd> PartialEq for IntervalLimit<T> {
  fn eq(&self, other: &Self) -> bool {
    self.partial_cmp(other) == Some(Ordering::Equal)
  }
}

impl<T: Display + Clone + Hash + Eq + Ord + PartialEq + PartialOrd> PartialOrd
  for IntervalLimit<T>
{
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    if self.value.is_limitless() && other.value.is_limitless() {
      if self.lower == other.lower {
        Some(Ordering::Equal)
      } else {
        self.lower_to_ordering(Some(Ordering::Less), Some(Ordering::Greater))
      }
    } else if self.value.is_limitless() {
      self.lower_to_ordering(Some(Ordering::Less), Some(Ordering::Greater))
    } else if other.value.is_limitless() {
      other.lower_to_ordering(Some(Ordering::Greater), Some(Ordering::Less))
    } else if self.value == other.value {
      if self.lower && other.lower {
        if self.closed ^ other.closed {
          self.closed_to_ordering(Some(Ordering::Less), Some(Ordering::Greater))
        } else {
          Some(Ordering::Equal)
        }
      } else if !self.lower && !other.lower {
        if self.closed ^ other.closed {
          self.closed_to_ordering(Some(Ordering::Greater), Some(Ordering::Less))
        } else {
          Some(Ordering::Equal)
        }
      } else {
        self.lower_to_ordering(Some(Ordering::Less), Some(Ordering::Greater))
      }
    } else {
      self.value.partial_cmp(&other.value)
    }
  }
}

impl<T: Display + Clone + Hash + Eq + Ord + PartialEq + PartialOrd> IntervalLimit<T> {
  pub fn is_closed(&self) -> bool {
    self.closed
  }

  pub fn is_lower(&self) -> bool {
    self.lower
  }

  pub fn get_value(&self) -> &LimitValue<T> {
    &self.value
  }

  pub fn new(closed: bool, lower: bool, value: LimitValue<T>) -> Self {
    Self {
      closed: if value.is_limitless() { false } else { closed },
      lower,
      value,
    }
  }

  pub fn lower(closed: bool, value: LimitValue<T>) -> Self {
    Self::new(closed, true, value)
  }

  pub fn upper(closed: bool, value: LimitValue<T>) -> Self {
    Self::new(closed, false, value)
  }

  fn lower_to_ordering<A>(&self, t: A, f: A) -> A {
    if self.lower {
      t
    } else {
      f
    }
  }

  fn closed_to_ordering<A>(&self, t: A, f: A) -> A {
    if self.closed {
      t
    } else {
      f
    }
  }

  pub fn is_infinity(&self) -> bool {
    self.value.is_limitless()
  }

  pub fn non_infinity(&self) -> bool {
    self.value.is_limit()
  }

  pub fn is_open(&self) -> bool {
    !self.closed
  }

  pub fn is_upper(&self) -> bool {
    !self.lower
  }
}

impl<T: Display + Clone + Hash + Eq + Ord + PartialEq + PartialOrd> Display for IntervalLimit<T> {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(
      f,
      "IntervalLimit({}, {}, {})",
      self.closed, self.lower, self.value
    )
  }
}
