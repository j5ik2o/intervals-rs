use std::fmt::{Debug, Display, Formatter};
use std::hash::Hash;

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
      self.as_lower_limit() == other.as_lower_limit()
    } else if self.is_single_element() ^ other.is_single_element() {
      false
    } else {
      self.upper == other.upper && self.lower == other.lower
    }
  }
}

impl<T: Debug + Display + Clone + Hash + Eq + Ord + PartialEq + PartialOrd> Interval<T> {

  /// Generate an interval.
  ///
  /// - params
  ///     - lower: lower interval limit
  ///     - upper: upper interval limit
  /// - return: an interval
  pub fn new(lower: IntervalLimit<T>, upper: IntervalLimit<T>) -> Interval<T> {
    Self::check_lower_is_less_than_or_equal_upper(&lower, &upper);
    let mut l = lower.clone();
    let mut u = upper.clone();
    if !upper.is_infinity()
        && !lower.is_infinity()
        && upper.as_value() == lower.as_value()
        && (lower.is_open() ^ upper.is_open())
    {
      if lower.is_open() {
        l = IntervalLimit::lower(true, lower.as_value().clone());
      }
      if upper.is_open() {
        u = IntervalLimit::upper(true, upper.as_value().clone());
      }
    }
    Self { lower: l, upper: u }
  }

  /// Generate an interval.
  ///
  /// Mainly used to generate half-open interval (intervals where only one of the upper and lower limits is open).
  ///
  /// - params
  ///     - lower: lower limit, Limitless means there is no limit.
  ///     - lower_included: specify `true` if the lower limit is included in the interval (closed lower limit).
  ///     - upper: upper limit, Limitless means there is no limit.
  ///     - upper_included: specify `true` if the upper limit is included in the interval (closed upper limit)
  /// - return: an interval
  /// - panic
  ///     - if the lower limit is greater than the upper limit
  pub fn over(
    lower: LimitValue<T>,
    lower_included: bool,
    upper: LimitValue<T>,
    upper_included: bool,
  ) -> Self {
    Self::new(
      IntervalLimit::lower(lower_included, lower),
      IntervalLimit::upper(upper_included, upper),
    )
  }

  /// Generate an interval with only the lower limit.
  ///
  /// The lower limit is the interval that is included (closed) in the interval.
  ///
  /// - params
  ///     - lower: lower limit, Limitless means that there is no limit.
  /// - return: an interval
  pub fn and_more(lower: LimitValue<T>) -> Self {
    Self::closed(lower, LimitValue::<T>::Limitless)
  }

  /// Generate a closed interval.
  ///
  /// - params
  ///     - lower: lower limit, Limitless means there is no limit.
  ///     - upper: upper limit, Limitless means there is no limit.
  /// - return: a closed interval
  /// - panic
  ///     - if the lower limit is greater than the upper limit
  pub fn closed(lower: LimitValue<T>, upper: LimitValue<T>) -> Self {
    Self::over(lower, true, upper, true)
  }

  /// Generate an interval with only the lower limit.
  ///
  /// The lower limit is the interval that is not included in the (open) interval.
  ///
  /// - params
  ///     - lower: lower limit, Limitless means there is no limit.
  /// - return: an interval
  pub fn more_than(lower: LimitValue<T>) -> Self {
    Self::open(lower, LimitValue::<T>::Limitless)
  }

  /// Generate an open interval.
  ///
  /// - params
  ///     - lower: lower limit, Limitless means there is no limit.
  ///     - upper: upper limit, Limitless means there is no limit.
  /// - return: an open interval
  pub fn open(lower: LimitValue<T>, upper: LimitValue<T>) -> Self {
    Self::over(lower, false, upper, false)
  }

  /// Generate a single-element interval.
  ///
  /// - params
  ///     - element: an limit value
  /// - return: an interval
  pub fn single_element(element: LimitValue<T>) -> Self {
    Self::closed(element.clone(), element)
  }

  /// Generate an interval with only an upper limit.
  ///
  /// The upper limit is the interval that is not included in the (open) interval.
  ///
  /// - params
  ///     - upper: upper limit, Limitless means there is no limit.
  /// - return: an interval
  pub fn under(upper: LimitValue<T>) -> Self {
    Self::open(LimitValue::<T>::Limitless, upper)
  }

  /// Generate an interval with only an upper limit.
  ///
  /// The upper limit is the (closed) interval included in the interval.
  ///
  /// - params
  ///     - upper: upper limit, Limitless means there is no limit.
  /// - return: an interval
  pub fn up_to(upper: LimitValue<T>) -> Self {
    Self::closed(LimitValue::<T>::Limitless, upper)
  }

  pub fn as_upper_limit(&self) -> &LimitValue<T> {
    self.upper.as_value()
  }

  pub fn as_lower_limit(&self) -> &LimitValue<T> {
    self.lower.as_value()
  }

  /// Verify that this interval completely encloses the specified interval `other`.
  ///
  /// - params
  ///     - other: an `Interval`
  /// - return: `true` for full comprehension, `false` otherwise
  pub fn covers(&self, other: &Interval<T>) -> bool {
    let lower_pass = self.includes(other.as_lower_limit())
      || self.as_lower_limit() == other.as_lower_limit() && !other.includes_lower_limit();
    let upper_pass = self.includes(other.as_upper_limit())
      || self.as_upper_limit() == other.as_upper_limit() && !other.includes_upper_limit();
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
      self.as_upper_limit() == self.as_lower_limit() && !self.is_empty()
    }
  }

  /// Generate a new open interval with the same limits as this interval.
  ///
  /// - return: a new interval
  pub fn empty_of_same_type(&self) -> Interval<T> {
    self.new_of_same_type(
      self.as_lower_limit().clone(),
      false,
      self.as_lower_limit().clone(),
      false,
    )
  }

  /// Generate a new interval with the same type as this interval.
  ///
  /// - params
  ///     - lower: lower limit, if there is no limit value, then Limitless.
  ///     - lower_closed: Specify `true` if the lower limit is included in the interval (closed lower limit).
  ///     - upper: upper limit, if there is no limit value, then Limitless.
  ///     - upper_closed: specify `true` if the upper limit is included in the interval (closed upper limit)
  /// - return: an new interval
  pub fn new_of_same_type(
    &self,
    lower: LimitValue<T>,
    lower_closed: bool,
    upper: LimitValue<T>,
    upper_closed: bool,
  ) -> Interval<T> {
    Self::over(lower, lower_closed, upper, upper_closed)
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
      *self.as_upper_limit() < *value
        || *self.as_upper_limit() == *value && !self.includes_upper_limit()
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
      *self.as_lower_limit() > *value
        || *self.as_lower_limit() == *value && !self.includes_lower_limit()
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
    match (self.as_upper_limit(), self.as_lower_limit()) {
      (&LimitValue::Limitless, &LimitValue::Limitless) => false,
      _ => self.is_open() && self.as_upper_limit() == self.as_lower_limit(),
    }
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
    if self.equal_both_limitless(self.as_upper_limit(), other.as_upper_limit()) {
      true
    } else if self.equal_both_limitless(self.as_lower_limit(), self.as_lower_limit()) {
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
    matches!(self.as_upper_limit(), LimitValue::Limit(_))
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
    matches!(self.as_lower_limit(), LimitValue::Limit(_))
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

  /// Get whether the lower limit is closed or not.
  ///
  /// Warning: This method is generally used for the purpose of displaying this value and for interaction with classes that are highly coupled to this class.
  /// Careless use of this method will unnecessarily increase the coupling between this class and the client-side class.
  ///
  /// If you want to use this value for calculations,
  /// - find another appropriate method or add a new method to this class.
  /// - find another suitable method or consider adding a new method to this class.
  ///
  /// - return: true` if the lower limit is closed, `false` otherwise
  pub fn includes_lower_limit(&self) -> bool {
    self.lower.is_closed()
  }

  pub(crate) fn complement_relative_to(&self, other: &Interval<T>) -> Vec<Interval<T>> {
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

  fn check_lower_is_less_than_or_equal_upper(lower: &IntervalLimit<T>, upper: &IntervalLimit<T>) {
    if !(lower.is_lower() && upper.is_upper() && lower <= upper) {
      panic!("{} is not before or equal to {}", lower, upper)
    }
  }

  fn equal_both_limitless(&self, me: &LimitValue<T>, your: &LimitValue<T>) -> bool {
    matches!((me, your), (&LimitValue::Limitless, &LimitValue::Limitless))
  }

  /// Return the larger (narrower marginal, more constrained) of the lower limits of this interval and the given interval `other`.
  ///
  /// - params
  ///     - other: limit value for comparison
  /// - return: greater limit value
  pub(crate) fn greater_of_lower_limits<'a>(&'a self, other: &'a Interval<T>) -> &'a LimitValue<T> {
    if *self.as_lower_limit() == LimitValue::Limitless {
      other.as_lower_limit()
    } else if *other.as_lower_limit() == LimitValue::Limitless {
      self.as_lower_limit()
    } else if self.as_lower_limit() >= other.as_lower_limit() {
      self.as_lower_limit()
    } else {
      other.as_lower_limit()
    }
  }

  /// Return the smaller (narrower marginal, more constrained) of the upper limits of this interval and the given interval `other`.
  ///
  /// - params
  ///     - other: limit value for comparison
  /// - return: lesser limit value
  pub(crate) fn lesser_of_upper_limits<'a>(&'a self, other: &'a Interval<T>) -> &'a LimitValue<T> {
    if *self.as_upper_limit() == LimitValue::Limitless {
      other.as_upper_limit()
    } else if *other.as_upper_limit() == LimitValue::Limitless {
      self.as_upper_limit()
    } else if self.as_upper_limit() <= other.as_upper_limit() {
      self.as_upper_limit()
    } else {
      other.as_upper_limit()
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

  /// この区間の下側補区間と与えた区間 `other` の共通部分を返す。
  ///
  /// other 比較対象の区間
  /// return この区間の下側の補区間と、与えた区間の共通部分。存在しない場合は `None`
  fn left_complement_relative_to(&self, other: &Interval<T>) -> Option<Interval<T>> {
    // この区間の下側限界値の方が小さいか等しい場合、下側の補区間に共通部分は無い
    if self.lower <= other.lower {
      None
    } else {
      Some(self.new_of_same_type(
        other.as_lower_limit().clone(),
        other.includes_lower_limit(),
        self.as_lower_limit().clone(),
        !self.includes_lower_limit(),
      ))
    }
  }

  fn right_complement_relative_to(&self, other: &Interval<T>) -> Option<Interval<T>> {
    // この区間の上側限界値の方が大きいか等しい場合、上側の補区間に共通部分は無い
    if self.upper >= other.upper {
      None
    } else {
      Some(self.new_of_same_type(
        self.as_upper_limit().clone(),
        !self.includes_upper_limit(),
        other.as_upper_limit().clone(),
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
      write!(f, "{{{}}}", self.as_lower_limit().to_string())
    } else {
      let mut str = String::new();
      if self.includes_lower_limit() {
        str.push('[');
      } else {
        str.push('(');
      }
      if self.has_lower_limit() {
        str.push_str(&self.as_lower_limit().to_string());
      } else {
        str.push_str(&"Infinity");
      }
      str.push_str(&", ");
      if self.has_upper_limit() {
        str.push_str(&self.as_upper_limit().to_string());
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
