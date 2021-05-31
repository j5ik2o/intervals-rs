use once_cell::sync::Lazy;

use crate::{Interval, LimitValue};
use rust_decimal::Decimal;
use rust_decimal::prelude::{FromPrimitive, ToPrimitive};

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
  let range = Interval::up_to(LimitValue::Limit(Decimal::from_f32(5.5).unwrap()));
  assert!(range.includes(&LimitValue::Limit(Decimal::from_f32(5.5).unwrap())));
  assert!(range.includes(&LimitValue::Limit(Decimal::from_f32(-5.5).unwrap())));
  assert!(range.includes(&LimitValue::Limit(Decimal::MIN)));
  assert!(!range.includes(&LimitValue::Limit(Decimal::from_f32(5.5001).unwrap())));
}

#[test]
fn test05_and_more() {
  let range = Interval::and_more(LimitValue::Limit(Decimal::from_f32(5.5).unwrap()));
  assert!(range.includes(&LimitValue::Limit(Decimal::from_f32(5.5).unwrap())));
  assert!(!range.includes(&LimitValue::Limit(Decimal::from_f32(5.4999).unwrap())));
  assert!(!range.includes(&LimitValue::Limit(Decimal::from_f32(-5.5).unwrap())));
  assert!(range.includes(&LimitValue::Limit(Decimal::MAX)));
  assert!(range.includes(&LimitValue::Limit(Decimal::from_f32(5.5001).unwrap())));
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
  let range = Interval::closed(
    LimitValue::Limit(Decimal::from_f32(-5.5).unwrap()),
    LimitValue::Limit(Decimal::from_f32(6.6).unwrap()),
  );
  assert!(!range.is_below(&LimitValue::Limit(Decimal::from_f32(5.0).unwrap())));
  assert!(!range.is_below(&LimitValue::Limit(Decimal::from_f32(-5.5).unwrap())));
  assert!(!range.is_below(&LimitValue::Limit(Decimal::from_f32(-5.4999).unwrap())));
  assert!(!range.is_below(&LimitValue::Limit(Decimal::from_f32(6.6).unwrap())));
  assert!(range.is_below(&LimitValue::Limit(Decimal::from_f32(6.601).unwrap())));
  assert!(!range.is_below(&LimitValue::Limit(Decimal::from_f32(-5.501).unwrap())));
}

#[test]
fn test08_includes() {
  let range = Interval::closed(
    LimitValue::Limit(Decimal::from_f32(-5.5).unwrap()),
    LimitValue::Limit(Decimal::from_f32(6.6).unwrap()),
  );
  assert!(range.includes(&LimitValue::Limit(Decimal::from_f32(5.0).unwrap())));
  assert!(range.includes(&LimitValue::Limit(Decimal::from_f32(-5.5).unwrap())));
  assert!(range.includes(&LimitValue::Limit(Decimal::from_f32(-5.4999).unwrap())));
  assert!(range.includes(&LimitValue::Limit(Decimal::from_f32(6.6).unwrap())));
  assert!(!range.includes(&LimitValue::Limit(Decimal::from_f32(6.601).unwrap())));
  assert!(!range.includes(&LimitValue::Limit(Decimal::from_f32(-5.501).unwrap())));
}

#[test]
fn test09_open_interval() {
  let ex_range = Interval::over(
    LimitValue::Limit(Decimal::from_f32(-5.5).unwrap()),
    false,
    LimitValue::Limit(Decimal::from_f32(6.6).unwrap()),
    true,
  );
  assert!(ex_range.includes(&LimitValue::Limit(Decimal::from_f32(5.0).unwrap())));
  assert!(!ex_range.includes(&LimitValue::Limit(Decimal::from_f32(-5.5).unwrap())));
  assert!(ex_range.includes(&LimitValue::Limit(Decimal::from_f32(-5.4999).unwrap())));
  assert!(ex_range.includes(&LimitValue::Limit(Decimal::from_f32(6.6).unwrap())));
  assert!(!ex_range.includes(&LimitValue::Limit(Decimal::from_f32(6.601).unwrap())));
  assert!(!ex_range.includes(&LimitValue::Limit(Decimal::from_f32(-5.501).unwrap())));
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

#[test]
fn test16_gap() {
  let c1_3c = Interval::closed(LimitValue::Limit(1), LimitValue::Limit(3));
  let c5_7c = Interval::closed(LimitValue::Limit(5), LimitValue::Limit(7));
  let o3_5o = Interval::open(LimitValue::Limit(3), LimitValue::Limit(5));
  let c2_3o = Interval::over(LimitValue::Limit(2), true, LimitValue::Limit(3), false);

  assert_eq!(c1_3c.gap(&c5_7c), o3_5o);
  assert!(c1_3c.gap(&o3_5o).is_empty());
  assert!(c1_3c.gap(&c2_3o).is_empty());
  assert!(c2_3o.gap(&o3_5o).is_single_element());
}

#[test]
fn test17_relative_complement_disjoint() {
  let c1_3c = Interval::closed(LimitValue::Limit(1), LimitValue::Limit(3));
  let c5_7c = Interval::closed(LimitValue::Limit(5), LimitValue::Limit(7));
  let complement = c1_3c.complement_relative_to(&c5_7c);
  assert_eq!(complement.len(), 1);
  assert_eq!(complement[0], c5_7c);
}

#[test]
fn test18_relative_complement_disjoint_adjacent_open() {
  let c1_3o = Interval::over(LimitValue::Limit(1), true, LimitValue::Limit(3), false);
  let c3_7c = Interval::closed(LimitValue::Limit(3), LimitValue::Limit(7));
  let complement = c1_3o.complement_relative_to(&c3_7c);
  assert_eq!(complement.len(), 1);
  assert_eq!(complement[0], c3_7c);
}

#[test]
fn test19_relative_complement_overlap_left() {
  let c1_5c = Interval::closed(LimitValue::Limit(1), LimitValue::Limit(5));
  let c3_7c = Interval::closed(LimitValue::Limit(3), LimitValue::Limit(7));
  let complement = c3_7c.complement_relative_to(&c1_5c);
  let c1_3o = Interval::over(LimitValue::Limit(1), true, LimitValue::Limit(3), false);
  assert_eq!(complement.len(), 1);
  assert_eq!(complement[0], c1_3o);
}

#[test]
fn test20_relative_complement_overlap_right() {
  let c1_5c = Interval::closed(LimitValue::Limit(1), LimitValue::Limit(5));
  let c3_7c = Interval::closed(LimitValue::Limit(3), LimitValue::Limit(7));
  let complement = c1_5c.complement_relative_to(&c3_7c);
  let o5_7c = Interval::over(LimitValue::Limit(5), false, LimitValue::Limit(7), true);
  assert_eq!(complement.len(), 1);
  assert_eq!(complement[0], o5_7c);
}

#[test]
fn test21_relative_complement_adjacent_closed() {
  let c1_3c = Interval::closed(LimitValue::Limit(1), LimitValue::Limit(3));
  let c5_7c = Interval::closed(LimitValue::Limit(5), LimitValue::Limit(7));
  let complement = c1_3c.complement_relative_to(&c5_7c);
  assert_eq!(complement.len(), 1);
  assert_eq!(complement[0], c5_7c);
}

#[test]
fn test22_relative_complement_enclosing() {
  let c3_5c = Interval::closed(LimitValue::Limit(3), LimitValue::Limit(5));
  let c1_7c = Interval::closed(LimitValue::Limit(1), LimitValue::Limit(7));
  let complement = c1_7c.complement_relative_to(&c3_5c);
  assert_eq!(complement.len(), 0);
}

#[test]
fn test23_relative_complement_equal() {
  let c1_7c = Interval::closed(LimitValue::Limit(1), LimitValue::Limit(7));
  let complement = c1_7c.complement_relative_to(&c1_7c);
  assert_eq!(complement.len(), 0);
}

#[test]
fn test24_relative_complement_enclosed() {
  let c3_5c = Interval::closed(LimitValue::Limit(3), LimitValue::Limit(5));
  let c1_7c = Interval::closed(LimitValue::Limit(1), LimitValue::Limit(7));
  let c1_3o = Interval::over(LimitValue::Limit(1), true, LimitValue::Limit(3), false);
  let o5_7c = Interval::over(LimitValue::Limit(5), false, LimitValue::Limit(7), true);
  let complement = c3_5c.complement_relative_to(&c1_7c);
  assert_eq!(complement.len(), 2);
  assert_eq!(complement[0], c1_3o);
  assert_eq!(complement[1], o5_7c);
}

#[test]
fn test25_relative_complement_enclosed_end_point() {
  let o3_5o = Interval::open(LimitValue::Limit(3), LimitValue::Limit(5));
  let c3_5c = Interval::closed(LimitValue::Limit(3), LimitValue::Limit(5));
  let complement = o3_5o.complement_relative_to(&c3_5c);
  assert_eq!(complement.len(), 2);
  assert!(complement[0].includes(&LimitValue::Limit(3)));
}

#[test]
fn test26_is_single_element() {
  assert!(o1_1c.is_single_element());
  assert!(c1_1c.is_single_element());
  assert!(c1_1o.is_single_element());
  assert!(!c1_10c.is_single_element());
  assert!(!o1_1o.is_single_element());
}

#[test]
fn test27_equals_for_one_point_intervals() {
  assert_eq!(*c1_1o, *o1_1c);
  assert_eq!(*c1_1c, *o1_1c);
  assert_eq!(*c1_1c, *c1_1o);
  assert_ne!(*o1_1c, *o1_1o);
}

#[test]
fn test28_equals_for_empty_intervals() {
  assert_eq!(c4_6c.empty_of_same_type(), c1_10c.empty_of_same_type())
}

#[test]
fn test29_relative_complement_enclosed_open() {
  let o3_5o = Interval::open(LimitValue::Limit(3), LimitValue::Limit(5));
  let c1_7c = Interval::closed(LimitValue::Limit(1), LimitValue::Limit(7));
  let c1_3c = Interval::closed(LimitValue::Limit(1), LimitValue::Limit(3));
  let c5_7c = Interval::closed(LimitValue::Limit(5), LimitValue::Limit(7));
  let complement = o3_5o.complement_relative_to(&c1_7c);
  assert_eq!(complement.len(), 2);
  assert_eq!(complement[0], c1_3c);
  assert_eq!(complement[1], c5_7c);
}

#[test]
fn test30_to_string() {
  assert_eq!(c1_10c.to_string(), "[Limit(1), Limit(10)]");
  assert_eq!(o10_12c.to_string(), "(Limit(10), Limit(12)]");
  assert_eq!(empty.to_string(), "{}");
  assert_eq!(
    Interval::closed(LimitValue::Limit(10), LimitValue::Limit(10)).to_string(),
    "{Limit(10)}"
  );
}

#[test]
fn test31_relative_complement_overlap_right_open() {
  let c3_7o = Interval::over(LimitValue::Limit(3), true, LimitValue::Limit(6), false);
  let c1_5o = Interval::over(LimitValue::Limit(1), true, LimitValue::Limit(5), false);
  let complement = c3_7o.complement_relative_to(&c1_5o);
  let c1_3o = Interval::over(LimitValue::Limit(1), true, LimitValue::Limit(3), false);
  assert_eq!(complement.len(), 1);
  assert_eq!(complement[0], c1_3o);
}

#[test]
fn test32_relative_complement_overlap_left_open() {
  let o1_5c = Interval::over(LimitValue::Limit(1), false, LimitValue::Limit(5), true);
  let o3_7c = Interval::over(LimitValue::Limit(2), false, LimitValue::Limit(7), true);
  let complement = o1_5c.complement_relative_to(&o3_7c);
  let o5_7c = Interval::over(LimitValue::Limit(5), false, LimitValue::Limit(7), true);
  assert_eq!(complement.len(), 1);
  assert_eq!(complement[0], o5_7c);
}
