
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
