use std::cmp::Ordering;
use std::fmt::{Debug, Display, Formatter};

use crate::interval_limit::IntervalLimit;
use crate::limit_value::LimitValue::Limitless;
use crate::LimitValue;

#[derive(Debug, Clone, PartialEq)]
pub struct Interval<T: Debug + Display + Clone + PartialEq + PartialOrd> {
    lower: IntervalLimit<T>,
    upper: IntervalLimit<T>,
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
        if intersect_lower_bound > intersect_upper_bound {
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
        if !(lower.is_lower()
            && upper.is_upper()
            && lower <= upper)
        {
            panic!("{} is not before or equal to {}", lower, upper)
        }
    }

    fn equal_both_limitless(&self, me: &LimitValue<T>, your: &LimitValue<T>) -> bool {
        match (me, your) {
            (&LimitValue::Limitless, &LimitValue::Limitless) => true,
            _ => false,
        }
    }

    fn greater_of_lower_limits<'a>(&'a self, other: &'a Interval<T>) -> &'a LimitValue<T> {
        if self.lower_limit() == &LimitValue::Limitless {
            other.lower_limit()
        } else if other.lower_limit() == &LimitValue::Limitless {
            self.lower_limit()
        } else if self.lower_limit() >= other.lower_limit() {
            self.lower_limit()
        } else {
            other.lower_limit()
        }
    }

    fn lesser_of_upper_limits<'a>(&'a self, other: &'a Interval<T>) -> &'a LimitValue<T> {
        if self.upper_limit() == &LimitValue::Limitless {
            other.upper_limit()
        } else if other.upper_limit() == &LimitValue::Limitless {
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
                str.push_str("[");
            } else {
                str.push_str("(");
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
                str.push_str("]");
            } else {
                str.push_str(")");
            }
            write!(f, "({})", str)
        }
    }
}

#[cfg(test)]
mod tests {
    use std::fmt::{Debug, Display};

    use once_cell::sync::Lazy;

    use crate::{Interval, LimitValue};

    static c5_10c: Lazy<Interval<i32>> = Lazy::new(|| Interval::closed(LimitValue::Limit(5), LimitValue::Limit(10)));
    static c1_10c: Lazy<Interval<i32>> = Lazy::new(|| Interval::closed(LimitValue::Limit(1), LimitValue::Limit(10)));
    static c4_6c: Lazy<Interval<i32>> = Lazy::new(|| Interval::closed(LimitValue::Limit(4), LimitValue::Limit(6)));
    static c5_15c: Lazy<Interval<i32>> = Lazy::new(|| Interval::closed(LimitValue::Limit(5), LimitValue::Limit(15)));
    static c12_16c: Lazy<Interval<i32>> = Lazy::new(|| Interval::closed(LimitValue::Limit(12), LimitValue::Limit(16)));
    static o10_12c: Lazy<Interval<i32>> = Lazy::new(|| Interval::over(LimitValue::Limit(10), false, LimitValue::Limit(12), true));
    static o1_1c: Lazy<Interval<i32>> = Lazy::new(|| Interval::over(LimitValue::Limit(1), false, LimitValue::Limit(1), true));
    static c1_1o: Lazy<Interval<i32>> = Lazy::new(|| Interval::over(LimitValue::Limit(1), true, LimitValue::Limit(1), false));
    static c1_1c: Lazy<Interval<i32>> = Lazy::new(|| Interval::over(LimitValue::Limit(1), true, LimitValue::Limit(1), true));
    static o1_1o: Lazy<Interval<i32>> = Lazy::new(|| Interval::over(LimitValue::Limit(1), false, LimitValue::Limit(1), false));

   /**
  val _2o: Interval[BigDecimal] = Interval.over(Limitless[BigDecimal], true, Limit(BigDecimal(2)), false)
  val o9_ : Interval[BigDecimal] = Interval.over(Limit(BigDecimal(9)), false, Limitless[BigDecimal], true)
  val empty: Interval[BigDecimal] = Interval.open(Limit(BigDecimal(1)), Limit(BigDecimal(1)))
  val all: Interval[BigDecimal] = Interval.closed(Limitless[BigDecimal], Limitless[BigDecimal])
  */

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
        let new_interval = concrete.new_of_same_type(LimitValue::Limit(1), false, LimitValue::Limit(4), false);
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
}
