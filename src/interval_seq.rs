use std::cmp::Ordering;
use std::fmt::{Debug, Display};
use std::hash::Hash;
use std::slice::Iter;

use crate::{Interval, IntervalLimit, to_ordering};

#[derive(Clone)]
pub enum Ordered {
  UpperLower {
    inverse_lower: bool,
    inverse_upper: bool,
  },
  LowerUpper {
    inverse_lower: bool,
    inverse_upper: bool,
  },
}

impl Ordered {
  fn lower_factor(&self) -> i8 {
    match self {
      Ordered::UpperLower { inverse_lower, .. } => {
        if *inverse_lower {
          -1
        } else {
          1
        }
      }
      Ordered::LowerUpper { inverse_lower, .. } => {
        if *inverse_lower {
          -1
        } else {
          1
        }
      }
    }
  }
  fn upper_factor(&self) -> i8 {
    match self {
      Ordered::UpperLower { inverse_upper, .. } => {
        if *inverse_upper {
          -1
        } else {
          1
        }
      }
      Ordered::LowerUpper { inverse_upper, .. } => {
        if *inverse_upper {
          -1
        } else {
          1
        }
      }
    }
  }

  pub fn compare<T>(&self, e1: &Interval<T>, e2: &Interval<T>) -> Ordering
  where
    T: Debug + Display + Clone + Hash + Eq + Ord + PartialEq + PartialOrd,
  {
    match self {
      Ordered::UpperLower { .. } => {
        if e1.is_empty() && e2.is_empty() {
          Ordering::Equal
        } else if e1.is_empty() {
          Ordering::Less
        } else if e2.is_empty() {
          Ordering::Greater
        } else {
          let upper_comparance = e1.upper.partial_cmp(&e2.upper).unwrap();
          let lower_comparance = e1.lower.partial_cmp(&e2.lower).unwrap();
          if upper_comparance != Ordering::Equal {
            to_ordering(upper_comparance as i8 * self.upper_factor())
          } else {
            to_ordering(lower_comparance as i8 * self.lower_factor())
          }
        }
      }
      Ordered::LowerUpper { .. } => {
        if e1.is_empty() && e2.is_empty() {
          Ordering::Equal
        } else if e1.is_empty() {
          Ordering::Greater
        } else if e2.is_empty() {
          Ordering::Less
        } else {
          let upper_comparance = e1.upper.partial_cmp(&e2.upper).unwrap();
          let lower_comparance = e1.lower.partial_cmp(&e2.lower).unwrap();
          if upper_comparance != Ordering::Equal {
            to_ordering(upper_comparance as i8 + self.lower_factor())
          } else {
            to_ordering(lower_comparance as i8 * self.upper_factor())
          }
        }
      }
    }
  }
}

/// A structure that represents an interval sequence (a sequence of multiple Intervals).
pub struct IntervalSeq<T: Debug + Display + Clone + Hash + Eq + Ord + PartialEq + PartialOrd> {
  /// interval sequence
  intervals: Vec<Interval<T>>,
  /// ordered
  ordered: Ordered,
}

impl<T: Debug + Display + Clone + Hash + Eq + Ord + PartialEq + PartialOrd> IntervalSeq<T> {

  /// Add an interval element to this interval sequence.
  ///
  /// - value: an interval
  pub fn append(&mut self, value: &Interval<T>) {
    self.intervals.push(value.clone());
  }

  /// Return whether the interval sequence are empty.
  ///
  /// return: true if the interval sequence are empty
  pub fn is_empty(&self) -> bool {
    self.intervals.is_empty()
  }

  /// Generate empty interval sequence.
  ///
  /// - return: `IntervalSeq`
  pub fn empty() -> Self {
    let intervals: Vec<Interval<T>> = vec![];
    Self::new(&intervals)
  }

  pub fn new(values: &[Interval<T>]) -> Self {
    let mut intervals: Vec<Interval<T>> = vec![];
    values.iter().for_each(|e| {
      intervals.push(e.clone());
    });
    Self {
      intervals,
      ordered: Ordered::UpperLower {
        inverse_lower: true,
        inverse_upper: false,
      },
    }
  }

  /// Return the smallest interval that encompasses all the element intervals.
  ///
  /// - return: the smallest interval that encompasses all the elemental intervals.
  /// - panic: if none of the elements are present
  pub fn extent(&self) -> Interval<T> {
    if self.intervals.is_empty() {
      panic!("")
    }
    let first = self.intervals.get(0).unwrap();
    if self.intervals.len() == 1 {
      first.clone()
    } else {
      let mut lowers = self
        .intervals
        .iter()
        .map(|e| e.lower.clone())
        .collect::<Vec<IntervalLimit<T>>>();
      lowers.sort_by(|a, b| a.partial_cmp(&b).unwrap());
      let lower = lowers.get(0).unwrap();
      let mut uppers = self
        .intervals
        .iter()
        .map(|e| e.upper.clone())
        .collect::<Vec<IntervalLimit<T>>>();
      uppers.sort_by(|a, b| b.partial_cmp(&a).unwrap());
      let upper = uppers.get(0).unwrap();
      first.new_of_same_type(
        lower.as_value().clone(),
        lower.is_closed(),
        upper.as_value().clone(),
        upper.is_closed(),
      )
    }
  }

  /// In the sorted intervals, return the intervals that are between adjacent intervals as the interval sequence.
  ///
  /// If the number of intervals is less than two, an empty sequence of intervals is returned.
  /// If the intervals overlap or touch each other, the intervals are not included in the result element.
  /// If all the intervals overlap, an empty interval sequence is returned.
  ///
  /// - return: gap interval sequence
  pub fn gap(&self) -> Self {
    if self.intervals.len() < 2 {
      let values: Vec<Interval<T>> = vec![];
      Self::new(&values)
    } else {
      let mut values: Vec<Interval<T>> = vec![];
      for i in 1usize..self.intervals.len() {
        let left = &self.intervals[i - 1];
        let right = &self.intervals[i];
        let gap = left.gap(right);
        if !gap.is_empty() {
          values.push(gap);
        }
      }
      Self::new(&values)
    }
  }

  /// Return the sorted intervals where adjacent intervals overlap each other as an interval sequence.
  ///
  /// If the number of intervals is less than two, an empty sequence of intervals is returned.
  /// If the intervals do not overlap or are tangent to each other, the intervals are not included in the result element.
  /// If all the intervals do not overlap, an empty interval sequence is returned.
  ///
  /// - return: common interval sequence
  pub fn intersections(&self) -> Self {
    if self.intervals.len() < 2 {
      let values: Vec<Interval<T>> = vec![];
      Self::new(&values)
    } else {
      let mut values: Vec<Interval<T>> = vec![];
      for i in 1usize..self.intervals.len() {
        let left = &self.intervals[i - 1];
        let right = &self.intervals[i];
        let gap = left.intersect(right);
        if !gap.is_empty() {
          values.push(gap);
        }
      }
      Self::new(&values)
    }
  }

  /// Gets an iterator of this interval sequence.
  pub fn iter(&mut self) -> Iter<Interval<T>> {
    let mut l = self.intervals.clone();
    l.sort_by(|a, b| self.ordered.compare(a, b));
    self.intervals = l;
    self.intervals.iter()
  }

  /// Gets the len of this interval sequence.
  pub fn len(&self) -> usize {
    self.intervals.len()
  }

  /// Gets the interval in this interval sequence by index
  pub fn get(&self, idx: usize) -> Option<&Interval<T>> {
    self.intervals.get(idx)
  }
}
