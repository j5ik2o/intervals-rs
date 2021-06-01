use std::cmp::Ordering;
use std::fmt::{Display, Formatter, Debug};

use crate::LimitValue;
use std::hash::Hash;

/// A struct that represents a "limit" in an interval.
///
/// In order to understand this struct, it is important to correctly recognize the distinction between `limit` and `value`.
/// The limit is the value represented by this struct `self`, and the value represented by `value` is the limit value.
///
/// When a limit is "closed", it means that the limit itself is not considered to be exceeded.
/// An "open" limit means that it is considered to be exceeded.
///
/// An infinite limit is an unbounded limit, and we express this state by saying that `value` is `Limitless`.
/// Infinite limits are always considered to be open.
/// Conversely, a limit that is not an infinite limit (one whose `value` is not `Limitless`) is called a finite limit.
///
/// The lower limit represents the limit where values below (or below) the limit are considered to be exceeded,
/// and the upper limit represents the limit where values above (or above) the limit are considered to be exceeded.
///
/// closed: if the limit is closed `true
/// lower: `true` for the lower limit, `false` for the upper limit
/// value: limit value, in the case of Limitless, it indicates that there is no limit.
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
  /// Verify whether this limit is closed or not.
  ///
  /// - return: if it is closed, `true`, otherwise `false`.
  pub fn is_closed(&self) -> bool {
    self.closed
  }

  /// Verify whether this limit is open or not.
  ///
  /// - return: if it is open, `true`, otherwise `false`.
  pub fn is_open(&self) -> bool {
    !self.closed
  }

  /// Verify whether or not this limit is the lower side limit.
  ///
  /// - return: `true` for an lower limit, `false` otherwise
  pub fn is_lower(&self) -> bool {
    self.lower
  }

  /// Verify whether or not this limit is the upper side limit.
  ///
  /// - return: `true` for an upper limit, `false` otherwise
  pub fn is_upper(&self) -> bool {
    !self.lower
  }

  /// Verify whether this limit is an infinite limit.
  ///
  /// - return: if it is an infinite limit, it is `true`, otherwise it is `false
  pub fn is_infinity(&self) -> bool {
    self.value.is_limitless()
  }

  /// Verify whether this limit is an finite limit.
  ///
  /// - return: if it is an finite limit, it is `true`, otherwise it is `false
  pub fn is_finite(&self) -> bool {
    self.value.is_limit()
  }

  /// Get limit value.
  ///
  /// - return: limit value
  pub fn as_value(&self) -> &LimitValue<T> {
    &self.value
  }

  /// Generate a limit
  ///
  /// - params
  ///     - closed: if the limit is closed `true
  ///     - lower: `true` for the lower limit, `false` for the upper limit
  ///     - value: limit value, in the case of Limitless, it indicates that there is no limit.
  /// - return: a new limit
  pub fn new(closed: bool, lower: bool, value: LimitValue<T>) -> Self {
    Self {
      closed: if value.is_limitless() { false } else { closed },
      lower,
      value,
    }
  }

  /// Generate a lower limit
  ///
  /// - params
  ///     - closed: if the limit is closed `true
  ///     - value: limit value, in the case of Limitless, it indicates that there is no limit.
  /// - return: a new limit
  pub fn lower(closed: bool, value: LimitValue<T>) -> Self {
    Self::new(closed, true, value)
  }

  /// Generate a upper limit
  ///
  /// - params
  ///     - closed: if the limit is closed `true
  ///     - value: limit value, in the case of Limitless, it indicates that there is no limit.
  /// - return: a new limit
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
