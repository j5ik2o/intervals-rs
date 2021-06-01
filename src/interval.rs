use std::cmp::Ordering;
use std::fmt::{Debug, Display, Formatter};
use std::hash::{Hash, Hasher};

use crate::interval_limit::IntervalLimit;
use crate::LimitValue;

#[derive(Debug, Clone, Hash, Eq)]
pub struct Interval<T: Debug + Display + Clone + Hash + Eq + Ord + PartialEq + PartialOrd> {
  pub(crate) lower: IntervalLimit<T>,
  pub(crate) upper: IntervalLimit<T>,
}

impl<T: Debug + Display + Clone + Hash + Eq + Ord + PartialEq + PartialOrd> PartialEq
  for Interval<T>
{
  /// Verify the identity of this interval and the given interval `other`.
  ///
  /// It returns `true` if both intervals are empty, and `false` if only one of them is empty.
  /// If both are single-element intervals, the limits that are single elements are compared with each other, and `true` is returned if they match.
  /// If only one of them is a single-element interval, `false` is returned.
  ///
  /// - param
  ///   - other: an interval to be compared
  /// - return: `true` if they are identical, `false` if they are not
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

impl<T: Debug + Display + Clone + Hash + Eq + Ord + PartialEq + PartialOrd> Interval<T> {
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

  /// Verify that this interval completely encloses the specified interval `other`.
  ///
  /// - params
  ///     - other: an `Interval`
  /// - return: `true` for full comprehension, `false` otherwise
  pub fn covers(&self, other: &Interval<T>) -> bool {
    let lower_pass = self.includes(other.lower_limit())
      || self.lower_limit() == other.lower_limit() && !other.includes_lower_limit();
    let upper_pass = self.includes(other.upper_limit())
      || self.upper_limit() == other.upper_limit() && !other.includes_upper_limit();
    lower_pass && upper_pass
  }

  /// Get the interval that lies between this interval and the given interval `other`.
  ///
  /// For example, the gap between [3, 5) and [10, 20) is [5, 19).
  /// If the two intervals have a common part, return an empty interval.
  ///
  /// - params
  ///     - other: an interval to be compared
  /// - return: gap interval
  pub fn gap(&self, other: &Interval<T>) -> Interval<T> {
    if self.intersects(other) {
      self.empty_of_same_type()
    } else {
      self.new_of_same_type(
        self.lesser_of_upper_limits(other).clone(),
        !self.lesser_of_upper_included_in_union(other),
        self.greater_of_lower_limits(other).clone(),
        !self.greater_of_lower_included_in_union(other),
      )
    }
  }

  /// Verify whether this interval is a single-element interval or not.
  ///
  /// A single-element interval has both upper and lower limits, and also indicates that these limits are equal and not an open interval.
  /// For example, `3 <= x < 3`, `3 <= x <= 3`, and `3 <= x <= 3`.
  ///
  /// - return: `true` if it's a single element interval, `false` otherwise
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

  /// Verify whether or not the specified value `value` is included in this interval.
  ///
  /// - params
  ///     - value: an interval value
  /// - return: `true` if included, `false` otherwise
  pub fn includes(&self, value: &LimitValue<T>) -> bool {
    !self.is_below(value) && !self.is_above(value)
  }

  /// Verify that the specified value `value` does not exceed the upper limit of this interval.
  ///
  /// - params
  ///     - value: an interval value
  /// - return: `true` if not exceeded, `false` otherwise
  pub fn is_below(&self, value: &LimitValue<T>) -> bool {
    if !self.has_upper_limit() {
      false
    } else {
      *self.upper_limit() < *value || *self.upper_limit() == *value && !self.includes_upper_limit()
    }
  }

  /// Verify that the specified value `value` does not exceed the lower limit of this interval.
  ///
  /// - params
  ///     - value: an interval value
  /// - return: `true` if not exceeded, `false` otherwise
  pub fn is_above(&self, value: &LimitValue<T>) -> bool {
    if !self.has_lower_limit() {
      false
    } else {
      *self.lower_limit() > *value || *self.lower_limit() == *value && !self.includes_lower_limit()
    }
  }

  /// Verify whether this interval is an open interval or not.
  ///
  /// - return: `true` if it's an open interval, `false` otherwise (including half-open interval)
  pub fn is_open(&self) -> bool {
    !self.includes_lower_limit() && !self.includes_upper_limit()
  }

  /// Verify whether this interval is a closed interval or not.
  ///
  /// - return: `true` if it's a closed interval, `false` otherwise (including half-open interval)
  pub fn is_closed(&self) -> bool {
    self.includes_upper_limit() && self.includes_lower_limit()
  }

  /// Verify whether this interval is empty or not.
  ///
  /// The interval is empty means that the upper and lower limits are the same value and the interval is open.
  /// For example, a state like `3 < x < 3`.
  ///
  /// - return: `true` if it's empty, `false` otherwise.
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
    if !upper.is_infinity()
      && !lower.is_infinity()
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

  /// Return the product set (common part) of this interval and the given interval `other`.
  ///
  /// If the common part does not exist, it returns an empty interval.
  ///
  /// - params
  ///     - other: an interval to be compared
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

  /// Verify if there is a common part between this interval and the given interval `other`.
  ///
  /// - params
  ///     - other: a target interval
  /// - return: `true` if the common part exists, `false` otherwise
  pub fn intersects(&self, other: &Interval<T>) -> bool {
    if self.equal_both_limitless(&self.upper_limit(), &other.upper_limit()) {
      true
    } else if self.equal_both_limitless(&self.lower_limit(), &self.lower_limit()) {
      true
    } else {
      let g = self.greater_of_lower_limits(other);
      let l = self.lesser_of_upper_limits(other);
      if g < l {
        true
      } else if g > l {
        false
      } else {
        self.greater_of_lower_included_in_intersection(other)
          && self.lesser_of_upper_included_in_intersection(other)
      }
    }
  }

  pub fn upper_limit(&self) -> &LimitValue<T> {
    self.upper.get_value()
  }
  pub fn lower_limit(&self) -> &LimitValue<T> {
    self.lower.get_value()
  }

  /// Get whether there is an upper limit or not.
  ///
  /// Warning: This method is generally used for the purpose of displaying this value and for interaction with classes that are highly coupled to this class.
  /// Careless use of this method will unnecessarily increase the coupling between this class and the client-side class.
  ///
  /// If you want to use this value for calculations,
  /// - find another appropriate method or add a new method to this class.
  /// - find another suitable method or consider adding a new method to this class.
  ///
  ///
  /// - return: `true` if upper limit is present, `false` otherwise
  pub fn has_upper_limit(&self) -> bool {
    match self.upper_limit() {
      LimitValue::Limit(_) => true,
      LimitValue::Limitless => false,
    }
  }

  /// Get whether there is an lower limit or not.
  ///
  /// Warning: This method is generally used for the purpose of displaying this value and for interaction with classes that are highly coupled to this class.
  /// Careless use of this method will unnecessarily increase the coupling between this class and the client-side class.
  ///
  /// If you want to use this value for calculations,
  /// - find another appropriate method or add a new method to this class.
  /// - find another suitable method or consider adding a new method to this class.
  ///
  ///
  /// - return: `true` if lower limit is present, `false` otherwise
  pub fn has_lower_limit(&self) -> bool {
    match self.lower_limit() {
      LimitValue::Limit(_) => true,
      LimitValue::Limitless => false,
    }
  }

  /// Get whether the upper limit is closed or not.
  ///
  /// Warning: This method is generally used for the purpose of displaying this value and for interaction with classes that are highly coupled to this class.
  /// Careless use of this method will unnecessarily increase the coupling between this class and the client-side class.
  ///
  /// If you want to use this value for calculations,
  /// - find another appropriate method or add a new method to this class.
  /// - find another suitable method or consider adding a new method to this class.
  ///
  /// - return: true` if the upper limit is closed, `false` otherwise
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

  /// この区間の下側補区間と与えた区間 `other` の共通部分を返す。
  ///
  /// other 比較対象の区間
  /// return この区間の下側の補区間と、与えた区間の共通部分。存在しない場合は `None`
  pub fn left_complement_relative_to(&self, other: &Interval<T>) -> Option<Interval<T>> {
    // この区間の下側限界値の方が小さいか等しい場合、下側の補区間に共通部分は無い
    if self.lower <= other.lower {
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
    if self.upper >= other.upper {
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

impl<T: Debug + Display + Clone + Hash + Eq + Ord + PartialEq + PartialOrd> Display
  for Interval<T>
{
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    if self.is_empty() {
      write!(f, "{{}}")
    } else if self.is_single_element() {
      write!(f, "{{{}}}", self.lower_limit().to_string())
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
      write!(f, "{}", str)
    }
  }
}
