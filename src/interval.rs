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

  pub fn new_of_same_type(
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

  pub fn greater_of_lower_limits<'a>(&'a self, other: &'a Interval<T>) -> &'a LimitValue<T> {
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

  pub fn lesser_of_upper_limits<'a>(&'a self, other: &'a Interval<T>) -> &'a LimitValue<T> {
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

  pub fn greater_of_lower_included_in_intersection(&self, other: &Interval<T>) -> bool {
    let limit = self.greater_of_lower_limits(other);
    self.includes(&limit) && other.includes(&limit)
  }

  pub fn greater_of_lower_included_in_union(&self, other: &Interval<T>) -> bool {
    let limit = self.greater_of_lower_limits(other);
    self.includes(&limit) || other.includes(&limit)
  }

  pub fn lesser_of_upper_included_in_intersection(&self, other: &Interval<T>) -> bool {
    let limit = self.lesser_of_upper_limits(other);
    self.includes(&limit) && other.includes(&limit)
  }

  pub fn lesser_of_upper_included_in_union(&self, other: &Interval<T>) -> bool {
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

