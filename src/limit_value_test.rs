use crate::LimitValue;

#[test]
fn it_works() {
    assert_eq!(LimitValue::Limit(1), LimitValue::Limit(1));
    assert!(LimitValue::Limit(1) < LimitValue::Limit(2));
    assert!(LimitValue::Limit(2) > LimitValue::Limit(1));
    assert_eq!(LimitValue::<i32>::Limitless, LimitValue::<i32>::Limitless);
    assert!(LimitValue::Limitless < LimitValue::Limit(1));
    assert!(LimitValue::Limit(1) > LimitValue::Limitless);
}