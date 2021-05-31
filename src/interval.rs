use std::cmp::Ordering;
use std::fmt::{Debug, Display, Formatter};

use crate::interval_limit::IntervalLimit;
use crate::LimitValue;

#[derive(Debug, Clone)]
pub struct Interval<T: Debug + Display + Clone + PartialEq + PartialOrd> {
  lower: IntervalLimit<T>,
  upper: IntervalLimit<T>,
}

impl<T: Debug + Display + Clone + PartialEq + PartialOrd> PartialEq for Interval<T> {
  fn eq(&self, other: &Self) -> bool {
    if self.is_empty() & other.is_empty() {
      true
    } else if self.is_empty() ^ other.is_empty() {
      false
    } else if self.is_single_element() & other.is_single_element() {
      self.lower_limit() == other.lower_limit()
    } else if self.is_single_element() ^ other.is_single_element() {
      false
    } else {
      self.upper == other.upper && self.lower == other.lower
    }
  }
}

impl<T: Debug + Display + Clone + PartialEq + PartialOrd> Interval<T> {
  pub fn and_more(lower: LimitValue<T>) -> Self {
    Self::closed(lower, LimitValue::<T>::Limitless)
  }

  pub fn closed(lower: LimitValue<T>, upper: LimitValue<T>) -> Self {
    Self::new_with(lower, true, upper, true)
  }

  pub fn more_than(lower: LimitValue<T>) -> Self {
    Self::open(lower, LimitValue::<T>::Limitless)
  }

  pub fn open(lower: LimitValue<T>, upper: LimitValue<T>) -> Self {
    Self::new_with(lower, false, upper, false)
  }

  pub fn over(
    lower: LimitValue<T>,
    lower_included: bool,
    upper: LimitValue<T>,
    upper_included: bool,
  ) -> Self {
    Self::new_with(lower, lower_included, upper, upper_included)
  }

  pub fn single_element(element: LimitValue<T>) -> Self {
    Self::closed(element.clone(), element)
  }

  pub fn under(upper: LimitValue<T>) -> Self {
    Self::open(LimitValue::<T>::Limitless, upper)
  }

  pub fn up_to(upper: LimitValue<T>) -> Self {
    Self::closed(LimitValue::<T>::Limitless, upper)
  }

  pub fn complement_relative_to(&self, other: &Interval<T>) -> Vec<Interval<T>> {
    let mut interval_sequence: Vec<Interval<T>> = vec![];
    if !self.intersects(other) {
      interval_sequence.push(other.clone());
      interval_sequence
    } else {
      if let Some(left) = self.left_complement_relative_to(other) {
        interval_sequence.push(left.clone());
      }
      if let Some(right) = self.right_complement_relative_to(other) {
        interval_sequence.push(right.clone());
      }
      interval_sequence
    }
  }

  pub fn covers(&self, other: &Interval<T>) -> bool {
    let lower_pass = self.includes(other.lower_limit())
      || self.lower_limit() == other.lower_limit() && !other.includes_lower_limit();
    let upper_pass = self.includes(other.upper_limit())
      || self.upper_limit() == other.upper_limit() && !other.includes_upper_limit();
    lower_pass && upper_pass
  }

  pub fn is_single_element(&self) -> bool {
    if !self.has_upper_limit() {
      false
    } else if !self.has_lower_limit() {
      false
    } else {
      self.upper_limit() == self.lower_limit() && !self.is_empty()
    }
  }

  pub fn empty_of_same_type(&self) -> Interval<T> {
    self.new_of_same_type(
      self.lower_limit().clone(),
      false,
      self.lower_limit().clone(),
      false,
    )
  }

  fn new_of_same_type(
    &self,
    lower: LimitValue<T>,
    lower_closed: bool,
    upper: LimitValue<T>,
    upper_closed: bool,
  ) -> Interval<T> {
    Self::new_with(lower, lower_closed, upper, upper_closed)
  }

  pub fn includes(&self, value: &LimitValue<T>) -> bool {
    !self.is_below(value) && !self.is_above(value)
  }

  pub fn is_below(&self, value: &LimitValue<T>) -> bool {
    if !self.has_upper_limit() {
      false
    } else {
      *self.upper_limit() < *value || *self.upper_limit() == *value && !self.includes_upper_limit()
    }
  }

  pub fn is_above(&self, value: &LimitValue<T>) -> bool {
    if !self.has_lower_limit() {
      false
    } else {
      *self.lower_limit() > *value || *self.lower_limit() == *value && !self.includes_lower_limit()
    }
  }

  pub fn is_open(&self) -> bool {
    !self.includes_lower_limit() && !self.includes_upper_limit()
  }

  pub fn is_closed(&self) -> bool {
    self.includes_upper_limit() && self.includes_lower_limit()
  }

  pub fn is_empty(&self) -> bool {
    match (self.upper_limit(), self.lower_limit()) {
      (&LimitValue::Limitless, &LimitValue::Limitless) => false,
      _ => self.is_open() && self.upper_limit() == self.lower_limit(),
    }
  }

  pub fn new(lower: IntervalLimit<T>, upper: IntervalLimit<T>) -> Interval<T> {
    Self::check_lower_is_less_than_or_equal_upper(&lower, &upper);
    let mut l = lower.clone();
    let mut u = upper.clone();
    if !upper.infinity()
      && !lower.infinity()
      && upper.get_value() == lower.get_value()
      && (lower.is_open() ^ upper.is_open())
    {
      if lower.is_open() {
        l = IntervalLimit::lower(true, lower.get_value().clone());
      }
      if upper.is_open() {
        u = IntervalLimit::upper(true, upper.get_value().clone());
      }
    }
    Self { lower: l, upper: u }
  }

  pub fn new_with(
    lower: LimitValue<T>,
    is_lower_closed: bool,
    upper: LimitValue<T>,
    is_upper_closed: bool,
  ) -> Interval<T> {
    Self::new(
      IntervalLimit::lower(is_lower_closed, lower),
      IntervalLimit::upper(is_upper_closed, upper),
    )
  }

  pub fn intersect(&self, other: &Interval<T>) -> Interval<T> {
    let intersect_lower_bound = self.greater_of_lower_limits(other);
    let intersect_upper_bound = self.lesser_of_upper_limits(other);
    if *intersect_lower_bound > *intersect_upper_bound {
      self.empty_of_same_type()
    } else {
      self.new_of_same_type(
        intersect_lower_bound.clone(),
        self.greater_of_lower_included_in_intersection(other),
        intersect_upper_bound.clone(),
        self.lesser_of_upper_included_in_intersection(other),
      )
    }
  }

  pub fn intersects(&self, other: &Interval<T>) -> bool {
    if self.equal_both_limitless(&self.upper_limit(), &other.upper_limit()) {
      true
    } else if self.equal_both_limitless(&self.lower_limit(), &self.upper_limit()) {
      true
    } else {
      match self
        .greater_of_lower_limits(other)
        .partial_cmp(&self.lesser_of_upper_limits(other))
      {
        Some(Ordering::Less) => true,
        Some(Ordering::Greater) => false,
        _ => {
          self.greater_of_lower_included_in_intersection(other)
            && self.lesser_of_upper_included_in_intersection(other)
        }
      }
    }
  }

  pub fn upper_limit(&self) -> &LimitValue<T> {
    self.upper.get_value()
  }
  pub fn lower_limit(&self) -> &LimitValue<T> {
    self.lower.get_value()
  }

  pub fn has_upper_limit(&self) -> bool {
    match self.upper_limit() {
      LimitValue::Limit(_) => true,
      LimitValue::Limitless => false,
    }
  }

  pub fn has_lower_limit(&self) -> bool {
    match self.lower_limit() {
      LimitValue::Limit(_) => true,
      LimitValue::Limitless => false,
    }
  }

  pub fn includes_upper_limit(&self) -> bool {
    self.upper.is_closed()
  }

  pub fn includes_lower_limit(&self) -> bool {
    self.lower.is_closed()
  }

  fn check_lower_is_less_than_or_equal_upper(lower: &IntervalLimit<T>, upper: &IntervalLimit<T>) {
    if !(lower.is_lower() && upper.is_upper() && lower <= upper) {
      panic!("{} is not before or equal to {}", lower, upper)
    }
  }

  fn equal_both_limitless(&self, me: &LimitValue<T>, your: &LimitValue<T>) -> bool {
    matches!((me, your), (&LimitValue::Limitless, &LimitValue::Limitless))
  }

  fn greater_of_lower_limits<'a>(&'a self, other: &'a Interval<T>) -> &'a LimitValue<T> {
    if *self.lower_limit() == LimitValue::Limitless {
      other.lower_limit()
    } else if *other.lower_limit() == LimitValue::Limitless {
      self.lower_limit()
    } else if self.lower_limit() >= other.lower_limit() {
      self.lower_limit()
    } else {
      other.lower_limit()
    }
  }

  fn lesser_of_upper_limits<'a>(&'a self, other: &'a Interval<T>) -> &'a LimitValue<T> {
    if *self.upper_limit() == LimitValue::Limitless {
      other.upper_limit()
    } else if *other.upper_limit() == LimitValue::Limitless {
      self.upper_limit()
    } else if self.upper_limit() <= other.upper_limit() {
      self.upper_limit()
    } else {
      other.upper_limit()
    }
  }

  fn greater_of_lower_included_in_intersection(&self, other: &Interval<T>) -> bool {
    let limit = self.greater_of_lower_limits(other);
    self.includes(&limit) && other.includes(&limit)
  }

  fn greater_of_lower_included_in_union(&self, other: &Interval<T>) -> bool {
    let limit = self.greater_of_lower_limits(other);
    self.includes(&limit) || other.includes(&limit)
  }

  fn lesser_of_upper_included_in_intersection(&self, other: &Interval<T>) -> bool {
    let limit = self.lesser_of_upper_limits(other);
    self.includes(&limit) && other.includes(&limit)
  }

  fn lesser_of_upper_included_in_union(&self, other: &Interval<T>) -> bool {
    let limit = self.lesser_of_upper_limits(other);
    self.includes(&limit) || other.includes(&limit)
  }

  /// この区間の下側<b>補</b>区間と与えた区間 `other` の共通部分を返す。
  ///
  /// other 比較対象の区間
  /// return この区間の下側の補区間と、与えた区間の共通部分。存在しない場合は `None`
  pub fn left_complement_relative_to(&self, other: &Interval<T>) -> Option<Interval<T>> {
    // この区間の下側限界値の方が小さいか等しい場合、下側の補区間に共通部分は無い
    if self.lower_limit().partial_cmp(other.lower_limit()) == Some(Ordering::Less) {
      None
    } else {
      Some(self.new_of_same_type(
        other.lower_limit().clone(),
        other.includes_lower_limit(),
        self.lower_limit().clone(),
        !self.includes_lower_limit(),
      ))
    }
  }

  pub fn right_complement_relative_to(&self, other: &Interval<T>) -> Option<Interval<T>> {
    // この区間の上側限界値の方が大きいか等しい場合、上側の補区間に共通部分は無い
    if self.upper_limit().partial_cmp(other.upper_limit()) == Some(Ordering::Greater) {
      None
    } else {
      Some(self.new_of_same_type(
        self.upper_limit().clone(),
        !self.includes_upper_limit(),
        other.upper_limit().clone(),
        other.includes_upper_limit(),
      ))
    }
  }
}

impl<T: Debug + Display + Clone + Eq + Ord + PartialEq + PartialOrd> Display for Interval<T> {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    if self.is_empty() {
      write!(f, "()")
    } else if self.is_single_element() {
      write!(f, "({})", self.lower_limit().to_string())
    } else {
      let mut str = String::new();
      if self.includes_lower_limit() {
        str.push('[');
      } else {
        str.push('(');
      }
      if self.has_lower_limit() {
        str.push_str(&self.lower_limit().to_string());
      } else {
        str.push_str(&"Infinity");
      }
      str.push_str(&", ");
      if self.has_upper_limit() {
        str.push_str(&self.upper_limit().to_string());
      } else {
        str.push_str(&"Infinity");
      }
      if self.includes_upper_limit() {
        str.push(']');
      } else {
        str.push(')');
      }
      write!(f, "({})", str)
    }
  }
}

#[cfg(test)]
mod tests {
  use once_cell::sync::Lazy;

  use crate::{Interval, LimitValue};

  static c5_10c: Lazy<Interval<i32>> =
    Lazy::new(|| Interval::closed(LimitValue::Limit(5), LimitValue::Limit(10)));
  static c1_10c: Lazy<Interval<i32>> =
    Lazy::new(|| Interval::closed(LimitValue::Limit(1), LimitValue::Limit(10)));
  static c4_6c: Lazy<Interval<i32>> =
    Lazy::new(|| Interval::closed(LimitValue::Limit(4), LimitValue::Limit(6)));
  static c5_15c: Lazy<Interval<i32>> =
    Lazy::new(|| Interval::closed(LimitValue::Limit(5), LimitValue::Limit(15)));
  static c12_16c: Lazy<Interval<i32>> =
    Lazy::new(|| Interval::closed(LimitValue::Limit(12), LimitValue::Limit(16)));
  static o10_12c: Lazy<Interval<i32>> =
    Lazy::new(|| Interval::over(LimitValue::Limit(10), false, LimitValue::Limit(12), true));
  static o1_1c: Lazy<Interval<i32>> =
    Lazy::new(|| Interval::over(LimitValue::Limit(1), false, LimitValue::Limit(1), true));
  static c1_1o: Lazy<Interval<i32>> =
    Lazy::new(|| Interval::over(LimitValue::Limit(1), true, LimitValue::Limit(1), false));
  static c1_1c: Lazy<Interval<i32>> =
    Lazy::new(|| Interval::over(LimitValue::Limit(1), true, LimitValue::Limit(1), true));
  static o1_1o: Lazy<Interval<i32>> =
    Lazy::new(|| Interval::over(LimitValue::Limit(1), false, LimitValue::Limit(1), false));
  static _2o: Lazy<Interval<i32>> =
    Lazy::new(|| Interval::over(LimitValue::Limitless, true, LimitValue::Limit(2), false));
  static o9_: Lazy<Interval<i32>> =
    Lazy::new(|| Interval::over(LimitValue::Limit(9), false, LimitValue::Limitless, true));
  static empty: Lazy<Interval<i32>> =
    Lazy::new(|| Interval::open(LimitValue::Limit(1), LimitValue::Limit(1)));
  static all: Lazy<Interval<i32>> =
    Lazy::new(|| Interval::closed(LimitValue::Limitless, LimitValue::Limitless));

  #[test]
  #[should_panic]
  fn test03_assertions() {
    Interval::closed(LimitValue::Limit(2), LimitValue::Limit(1));
  }

  #[test]
  fn test04_up_to() {
    let range = Interval::up_to(LimitValue::Limit(5.5f32));
    assert!(range.includes(&LimitValue::Limit(5.5f32)));
    assert!(range.includes(&LimitValue::Limit(-5.5f32)));
    assert!(range.includes(&LimitValue::Limit(f32::MIN)));
    assert!(!range.includes(&LimitValue::Limit(5.5001f32)));
  }

  #[test]
  fn test05_and_more() {
    let range = Interval::and_more(LimitValue::Limit(5.5));
    assert!(range.includes(&LimitValue::Limit(5.5)));
    assert!(!range.includes(&LimitValue::Limit(5.4999)));
    assert!(!range.includes(&LimitValue::Limit(-5.5)));
    assert!(range.includes(&LimitValue::Limit(f32::MAX)));
    assert!(range.includes(&LimitValue::Limit(5.5001)));
  }

  #[test]
  fn test06_abstract_creation() {
    let concrete = Interval::new_with(LimitValue::Limit(1), true, LimitValue::Limit(3), true);
    let new_interval =
      concrete.new_of_same_type(LimitValue::Limit(1), false, LimitValue::Limit(4), false);
    let expected = Interval::new_with(LimitValue::Limit(1), false, LimitValue::Limit(4), false);
    assert_eq!(new_interval, expected);
  }

  #[test]
  fn test07_below() {
    let range = Interval::closed(LimitValue::Limit(-5.5), LimitValue::Limit(6.6));
    assert!(!range.is_below(&LimitValue::Limit(5.0)));
    assert!(!range.is_below(&LimitValue::Limit(-5.5)));
    assert!(!range.is_below(&LimitValue::Limit(-5.4999)));
    assert!(!range.is_below(&LimitValue::Limit(6.6)));
    assert!(range.is_below(&LimitValue::Limit(6.601)));
    assert!(!range.is_below(&LimitValue::Limit(-5.501)));
  }

  #[test]
  fn test08_includes() {
    let range = Interval::closed(LimitValue::Limit(-5.5), LimitValue::Limit(6.6));
    assert!(range.includes(&LimitValue::Limit(5.0)));
    assert!(range.includes(&LimitValue::Limit(-5.5)));
    assert!(range.includes(&LimitValue::Limit(-5.4999)));
    assert!(range.includes(&LimitValue::Limit(6.6)));
    assert!(!range.includes(&LimitValue::Limit(6.601)));
    assert!(!range.includes(&LimitValue::Limit(-5.501)));
  }

  #[test]
  fn test09_open_interval() {
    let ex_range = Interval::over(LimitValue::Limit(-5.5), false, LimitValue::Limit(6.6), true);
    assert!(ex_range.includes(&LimitValue::Limit(5.0)));
    assert!(!ex_range.includes(&LimitValue::Limit(-5.5)));
    assert!(ex_range.includes(&LimitValue::Limit(-5.4999)));
    assert!(ex_range.includes(&LimitValue::Limit(6.6)));
    assert!(!ex_range.includes(&LimitValue::Limit(6.601)));
    assert!(!ex_range.includes(&LimitValue::Limit(-5.501)));
  }

  #[test]
  fn test10_is_empty() {
    assert!(!Interval::closed(LimitValue::Limit(5), LimitValue::Limit(6)).is_empty());
    assert!(!Interval::closed(LimitValue::Limit(6), LimitValue::Limit(6)).is_empty());
    assert!(Interval::open(LimitValue::Limit(6), LimitValue::Limit(6)).is_empty());
    assert!(c1_10c.empty_of_same_type().is_empty());
  }

  #[test]
  fn test11_intersects() {
    assert!(c5_10c.intersects(&c1_10c));
    assert!(c1_10c.intersects(&c5_10c));
    assert!(c4_6c.intersects(&c1_10c));
    assert!(c1_10c.intersects(&c4_6c));
    assert!(c5_10c.intersects(&c5_15c));
    assert!(c5_15c.intersects(&c1_10c));
    assert!(c1_10c.intersects(&c5_15c));
    assert!(!c1_10c.intersects(&c12_16c));
    assert!(!c12_16c.intersects(&c1_10c));
    assert!(c5_10c.intersects(&c5_10c));
    assert!(!c1_10c.intersects(&o10_12c));
    assert!(!o10_12c.intersects(&c1_10c));

    assert!(c5_10c.intersects(&c5_10c));
    assert!(c5_10c.intersects(&c1_10c));
    assert!(c5_10c.intersects(&c4_6c));
    assert!(c5_10c.intersects(&c5_15c));
    assert!(!c5_10c.intersects(&c12_16c));
    assert!(!c5_10c.intersects(&o10_12c));
    assert!(!c5_10c.intersects(&o1_1c));
    assert!(!c5_10c.intersects(&c1_1o));
    assert!(!c5_10c.intersects(&c1_1c));
    assert!(!c5_10c.intersects(&o1_1o));
    assert!(!c5_10c.intersects(&_2o));
    assert!(c5_10c.intersects(&o9_));
    assert!(!c5_10c.intersects(&empty));
    assert!(c5_10c.intersects(&all));

    assert!(c1_10c.intersects(&c5_10c));
    assert!(c1_10c.intersects(&c1_10c));
    assert!(c1_10c.intersects(&c4_6c));
    assert!(c1_10c.intersects(&c5_15c));
    assert!(!c1_10c.intersects(&c12_16c));
    assert!(!c1_10c.intersects(&o10_12c));
    assert!(c1_10c.intersects(&o1_1c));
    assert!(c1_10c.intersects(&c1_1o));
    assert!(c1_10c.intersects(&c1_1c));
    assert!(!c1_10c.intersects(&o1_1o));
    assert!(c1_10c.intersects(&_2o));
    assert!(c1_10c.intersects(&o9_));
    assert!(!c1_10c.intersects(&empty));
    assert!(c1_10c.intersects(&all));

    assert!(c4_6c.intersects(&c5_10c));
    assert!(c4_6c.intersects(&c1_10c));
    assert!(c4_6c.intersects(&c4_6c));
    assert!(c4_6c.intersects(&c5_15c));
    assert!(!c4_6c.intersects(&c12_16c));
    assert!(!c4_6c.intersects(&o10_12c));
    assert!(!c4_6c.intersects(&o1_1c));
    assert!(!c4_6c.intersects(&c1_1o));
    assert!(!c4_6c.intersects(&c1_1c));
    assert!(!c4_6c.intersects(&o1_1o));
    assert!(!c4_6c.intersects(&_2o));
    assert!(!c4_6c.intersects(&o9_));
    assert!(!c4_6c.intersects(&empty));
    assert!(c4_6c.intersects(&all));

    assert!(c5_15c.intersects(&c5_10c));
    assert!(c5_15c.intersects(&c1_10c));
    assert!(c5_15c.intersects(&c4_6c));
    assert!(c5_15c.intersects(&c5_15c));
    assert!(c5_15c.intersects(&c12_16c));
    assert!(c5_15c.intersects(&o10_12c));
    assert!(!c5_15c.intersects(&o1_1c));
    assert!(!c5_15c.intersects(&c1_1o));
    assert!(!c5_15c.intersects(&c1_1c));
    assert!(!c5_15c.intersects(&o1_1o));
    assert!(!c5_15c.intersects(&_2o));
    assert!(c5_15c.intersects(&o9_));
    assert!(!c5_15c.intersects(&empty));
    assert!(c5_15c.intersects(&all));

    assert!(!c12_16c.intersects(&c1_10c));
    assert!(!o10_12c.intersects(&c1_10c));
    assert!(!o1_1c.intersects(&c4_6c));
    assert!(!c1_1o.intersects(&c5_15c));
    assert!(!c1_1c.intersects(&c5_15c));
    assert!(!o1_1o.intersects(&c12_16c));
    assert!(!empty.intersects(&o10_12c));
    assert!(all.intersects(&o10_12c));
  }

  #[test]
  fn test12_intersection() {
    assert_eq!(c5_10c.intersect(&c1_10c), *c5_10c);
    assert_eq!(c1_10c.intersect(&c5_10c), *c5_10c);
    assert_eq!(c4_6c.intersect(&c1_10c), *c4_6c);
    assert_eq!(c1_10c.intersect(&c4_6c), *c4_6c);
    assert_eq!(c5_10c.intersect(&c5_15c), *c5_10c);
    assert_eq!(c5_15c.intersect(&c1_10c), *c5_10c);
    assert_eq!(c1_10c.intersect(&c5_15c), *c5_10c);
    assert!(c1_10c.intersect(&c12_16c).is_empty());
    assert_eq!(c1_10c.intersect(&c12_16c), *empty);
    assert_eq!(c12_16c.intersect(&c1_10c), *empty);
    assert_eq!(c5_10c.intersect(&c5_10c), *c5_10c);
    assert_eq!(c1_10c.intersect(&o10_12c), *empty);
    assert_eq!(o10_12c.intersect(&c1_10c), *empty);
  }

  #[test]
  fn test13_greater_of_lower_limits() {
    assert_eq!(
      c5_10c.greater_of_lower_limits(&c1_10c),
      &LimitValue::Limit(5)
    );
    assert_eq!(
      c1_10c.greater_of_lower_limits(&c5_10c),
      &LimitValue::Limit(5)
    );
    assert_eq!(
      c1_10c.greater_of_lower_limits(&c12_16c),
      &LimitValue::Limit(12)
    );
    assert_eq!(
      c12_16c.greater_of_lower_limits(&c1_10c),
      &LimitValue::Limit(12)
    );
  }

  #[test]
  fn test14_lesser_of_upper_limits() {
    assert_eq!(
      c5_10c.lesser_of_upper_limits(&c1_10c),
      &LimitValue::Limit(10)
    );
    assert_eq!(
      c1_10c.lesser_of_upper_limits(&c5_10c),
      &LimitValue::Limit(10)
    );
    assert_eq!(
      c4_6c.lesser_of_upper_limits(&c12_16c),
      &LimitValue::Limit(6)
    );
    assert_eq!(
      c12_16c.lesser_of_upper_limits(&c4_6c),
      &LimitValue::Limit(6)
    );
  }

  #[test]
  fn test15_covers_interval() {
    assert!(!c5_10c.covers(&c1_10c));
    assert!(c1_10c.covers(&c5_10c));
    assert!(!c4_6c.covers(&c1_10c));
    assert!(c1_10c.covers(&c4_6c));
    assert!(c5_10c.covers(&c5_10c));

    let o5_10c = Interval::over(LimitValue::Limit(5), false, LimitValue::Limit(10), true);
    assert!(c5_10c.covers(&o5_10c)); // "isClosed incl left-isOpen"
    assert!(o5_10c.covers(&o5_10c)); // "left-isOpen incl left-isOpen"
    assert!(!o5_10c.covers(&c5_10c)); // "left-isOpen doesn't include isClosed"

    let o1_10o = Interval::over(LimitValue::Limit(1), false, LimitValue::Limit(10), false);
    assert!(!c5_10c.covers(&o1_10o));
    assert!(o1_10o.covers(&o1_10o));
    assert!(!o1_10o.covers(&c5_10c));
  }
}
