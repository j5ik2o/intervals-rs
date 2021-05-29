use std::cmp::Ordering;

use crate::interval_limit::IntervalLimit;
use crate::limit_value::LimitValue::Limitless;
use crate::LimitValue;

#[derive(Debug)]
pub struct Interval<T: Clone + Eq + Ord + PartialEq + PartialOrd> {
    lower: IntervalLimit<T>,
    upper: IntervalLimit<T>,
}

impl<T: Clone + Eq + Ord + PartialEq + PartialOrd> Interval<T> {
    pub fn is_above(&self, value: &LimitValue<T>) -> bool {
        if !self.has_lower_limit() {
            false
        } else {
            self.lower_limit() > value || self.lower_limit() == value && !self.includes_lower_limit()
        }
    }

    pub fn is_below(&self, value: &LimitValue<T>) -> bool {
        if self.has_upper_limit() {
            false
        } else {
            self.upper_limit() < value || self.upper_limit() == value && !self.includes_upper_limit()
        }
    }

    pub fn includes(&self, value: &LimitValue<T>) -> bool {
        !self.is_below(value) && !self.is_above(value)
    }

    fn new(lower: LimitValue<T>, is_lower_closed: bool, upper: LimitValue<T>, is_upper_closed: bool) -> Interval<T> {
        Self {
            lower: IntervalLimit::lower(is_lower_closed, lower),
            upper: IntervalLimit::upper(is_upper_closed, upper),
        }
    }

    fn new_of_same_type(&self, lower: LimitValue<T>, lower_closed: bool,
                        upper: LimitValue<T>, upper_closed: bool) -> Interval<T> {
        Self::new(lower, lower_closed, upper, upper_closed)
    }

    pub fn empty_of_same_type(&self) -> Interval<T> {
        self.new_of_same_type(self.lower_limit().clone(), false, self.upper_limit().clone(), false)
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
            match self.greater_of_lower_limits(other).partial_cmp(&self.lesser_of_upper_limits(other)) {
                Some(Ordering::Less) => true,
                Some(Ordering::Greater) => false,
                _ =>
                    self.greater_of_lower_included_in_intersection(other) && self.lesser_of_upper_included_in_intersection(other),
            }
        }
    }

    fn equal_both_limitless(&self, me: &LimitValue<T>, your: &LimitValue<T>) -> bool {
        match (me, your) {
            (LimitValue::Limitless, LimitValue::Limitless) => true,
            _ => false
        }
    }

    pub fn upper_limit(&self) -> &LimitValue<T> { self.upper.get_value() }
    pub fn lower_limit(&self) -> &LimitValue<T> { self.lower.get_value() }

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
        self.upper.get_closed()
    }

    pub fn includes_lower_limit(&self) -> bool {
        self.lower.get_closed()
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
        let limit = self.greater_of_lower_limits(other.clone());
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
}