use std::cmp::Ordering;
use std::convert::TryFrom;
use std::fmt::{Debug, Display};
use std::hash::Hash;
use std::slice::Iter;

use crate::{Interval, IntervalLimit};

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

pub fn to_ordering(n: i8) -> Ordering {
  match n {
    -1 => Ordering::Less,
    0 => Ordering::Equal,
    1 => Ordering::Greater,
    _ => panic!(),
  }
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

pub struct IntervalSeq<T: Debug + Display + Clone + Hash + Eq + Ord + PartialEq + PartialOrd> {
  intervals: Vec<Interval<T>>,
  ordered: Ordered,
}

impl<T: Debug + Display + Clone + Hash + Eq + Ord + PartialEq + PartialOrd> IntervalSeq<T> {
  pub fn append(&mut self, value: &Interval<T>) {
    self.intervals.push(value.clone());
  }

  pub fn is_empty(&self) -> bool {
    self.intervals.is_empty()
  }

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
        lower.get_value().clone(),
        lower.is_closed(),
        upper.get_value().clone(),
        upper.is_closed(),
      )
    }
  }

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

  pub fn iter(&mut self) -> Iter<Interval<T>> {
    let mut l = self.intervals.clone();
    l.sort_by(|a, b| self.ordered.compare(a, b));
    self.intervals = l;
    self.intervals.iter()
  }

  pub fn len(&self) -> usize {
    self.intervals.len()
  }

  pub fn get(&self, idx: usize) -> Option<&Interval<T>> {
    self.intervals.get(idx)
  }
}

#[cfg(test)]
mod tests {
  use once_cell::sync::Lazy;

  use crate::{Interval, LimitValue};
  use crate::interval_seq::IntervalSeq;

  static c5_10c: Lazy<Interval<i32>> =
    Lazy::new(|| Interval::closed(LimitValue::Limit(5), LimitValue::Limit(10)));
  static o10_12c: Lazy<Interval<i32>> =
    Lazy::new(|| Interval::over(LimitValue::Limit(10), false, LimitValue::Limit(12), true));
  static o11_20c: Lazy<Interval<i32>> =
    Lazy::new(|| Interval::over(LimitValue::Limit(11), false, LimitValue::Limit(20), true));
  static o12_20o: Lazy<Interval<i32>> =
    Lazy::new(|| Interval::open(LimitValue::Limit(12), LimitValue::Limit(20)));
  static c20_25c: Lazy<Interval<i32>> =
    Lazy::new(|| Interval::closed(LimitValue::Limit(20), LimitValue::Limit(25)));
  static o25_30c: Lazy<Interval<i32>> =
    Lazy::new(|| Interval::over(LimitValue::Limit(25), false, LimitValue::Limit(30), true));
  static o11_12c: Lazy<Interval<i32>> =
    Lazy::new(|| Interval::over(LimitValue::Limit(11), false, LimitValue::Limit(12), true));
  static c20_20c: Lazy<Interval<i32>> =
    Lazy::new(|| Interval::closed(LimitValue::Limit(20), LimitValue::Limit(20)));
  static o30_35o: Lazy<Interval<i32>> =
    Lazy::new(|| Interval::open(LimitValue::Limit(30), LimitValue::Limit(35)));
  static _o18: Lazy<Interval<i32>> = Lazy::new(|| Interval::under(LimitValue::Limit(18)));
  static empty: Lazy<Interval<i32>> =
    Lazy::new(|| Interval::closed(LimitValue::Limit(0), LimitValue::Limit(0)));
  static all: Lazy<Interval<i32>> =
    Lazy::new(|| Interval::open(LimitValue::Limitless, LimitValue::Limitless));

  #[test]
  fn test01_iterate() {
    let mut interval_sequence = IntervalSeq::empty();
    interval_sequence.append(&empty);
    interval_sequence.append(&c5_10c);
    interval_sequence.append(&o10_12c);
    let mut iter = interval_sequence.iter();
    let next = iter.next();
    assert!(next.is_some());
    assert_eq!(next.unwrap(), &*empty);
    let next = iter.next();
    assert!(next.is_some());
    assert_eq!(next.unwrap(), &*c5_10c);
    let next = iter.next();
    assert!(next.is_some());
    assert_eq!(next.unwrap(), &*o10_12c);
    let next = iter.next();
    assert!(next.is_none());
  }

  #[test]
  fn test02_inserted_out_of_order() {
    let mut interval_sequence = IntervalSeq::empty();
    interval_sequence.append(&o10_12c);
    interval_sequence.append(&c5_10c);
    let mut iter = interval_sequence.iter();
    let next = iter.next();
    assert!(next.is_some());
    assert_eq!(next.unwrap(), &*c5_10c);
    let next = iter.next();
    assert!(next.is_some());
    assert_eq!(next.unwrap(), &*o10_12c);
    let next = iter.next();
    assert!(next.is_none());
  }

  #[test]
  fn test03_overlapping() {
    let mut interval_sequence = IntervalSeq::empty();
    interval_sequence.append(&o10_12c);
    interval_sequence.append(&o11_20c);
    let mut iter = interval_sequence.iter();
    let next = iter.next();
    assert!(next.is_some());
    assert_eq!(next.unwrap(), &*o10_12c);
    let next = iter.next();
    assert!(next.is_some());
    assert_eq!(next.unwrap(), &*o11_20c);
    let next = iter.next();
    assert!(next.is_none());
  }

  #[test]
  fn test04_intersections() {
    let mut interval_sequence = IntervalSeq::empty();
    interval_sequence.append(&o10_12c);
    interval_sequence.append(&o11_20c);
    interval_sequence.append(&c20_25c);
    let mut interval_sequence = interval_sequence.intersections();
    let mut iter = interval_sequence.iter();
    let next = iter.next();
    assert!(next.is_some());
    assert_eq!(next.unwrap(), &*o11_12c);
    let next = iter.next();
    assert!(next.is_some());
    assert_eq!(next.unwrap(), &*c20_20c);
    let next = iter.next();
    assert!(next.is_none());
  }

  #[test]
  fn test05_gaps() {
    let mut interval_sequence = IntervalSeq::empty();
    interval_sequence.append(&c5_10c);
    interval_sequence.append(&o10_12c);
    interval_sequence.append(&c20_25c);
    interval_sequence.append(&o30_35o);
    let mut interval_sequence = interval_sequence.gap();
    let mut iter = interval_sequence.iter();
    let next = iter.next();
    assert!(next.is_some());
    assert_eq!(next.unwrap(), &*o12_20o);
    let next = iter.next();
    assert!(next.is_some());
    assert_eq!(next.unwrap(), &*o25_30c);
    let next = iter.next();
    assert!(next.is_none());
  }

  #[test]
  fn test06_extent() {
    let mut values: Vec<Interval<i32>> = Vec::new();
    values.push(c5_10c.clone());
    values.push(o10_12c.clone());
    values.push(c20_25c.clone());

    let interval_sequence1 = IntervalSeq::new(&values);
    assert_eq!(
      interval_sequence1.extent(),
      Interval::closed(LimitValue::Limit(5), LimitValue::Limit(25))
    );

    values.push(_o18.clone());
    let interval_sequence2 = IntervalSeq::new(&values);
    assert_eq!(
      interval_sequence2.extent(),
      Interval::closed(LimitValue::Limitless, LimitValue::Limit(25))
    );

    values.push(all.clone());
    let interval_sequence3 = IntervalSeq::new(&values);
    assert_eq!(interval_sequence3.extent(), *all);
  }
}
