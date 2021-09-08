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

  let interval_sequence1 = IntervalSeq::new(values.clone());
  assert_eq!(
    interval_sequence1.extent(),
    Interval::closed(LimitValue::Limit(5), LimitValue::Limit(25))
  );

  values.push(_o18.clone());
  let interval_sequence2 = IntervalSeq::new(values.clone());
  assert_eq!(
    interval_sequence2.extent(),
    Interval::closed(LimitValue::Limitless, LimitValue::Limit(25))
  );

  values.push(all.clone());
  let interval_sequence3 = IntervalSeq::new(values);
  assert_eq!(interval_sequence3.extent(), *all);
}
