use rand::seq::SliceRandom;

use crate::interval_limit::IntervalLimit;
use crate::LimitValue;

#[test]
fn test01_equals() {
  assert_eq!(
    IntervalLimit::lower(false, LimitValue::Limit(10)),
    IntervalLimit::lower(false, LimitValue::Limit(10))
  );
  assert_ne!(
    IntervalLimit::lower(true, LimitValue::Limit(10)),
    IntervalLimit::lower(false, LimitValue::Limit(10))
  );
  assert_ne!(
    IntervalLimit::lower(false, LimitValue::Limit(10)),
    IntervalLimit::lower(true, LimitValue::Limit(10))
  );
  assert_eq!(
    IntervalLimit::lower(true, LimitValue::Limit(10)),
    IntervalLimit::lower(true, LimitValue::Limit(10))
  );

  assert_eq!(
    IntervalLimit::upper(false, LimitValue::Limit(10)),
    IntervalLimit::upper(false, LimitValue::Limit(10))
  );
  assert_ne!(
    IntervalLimit::upper(true, LimitValue::Limit(10)),
    IntervalLimit::upper(false, LimitValue::Limit(10))
  );
  assert_ne!(
    IntervalLimit::upper(false, LimitValue::Limit(10)),
    IntervalLimit::upper(true, LimitValue::Limit(10))
  );
  assert_eq!(
    IntervalLimit::upper(true, LimitValue::Limit(10)),
    IntervalLimit::upper(true, LimitValue::Limit(10))
  );

  assert_ne!(
    IntervalLimit::lower(false, LimitValue::Limit(10)),
    IntervalLimit::upper(false, LimitValue::Limit(10))
  );
  assert_ne!(
    IntervalLimit::lower(true, LimitValue::Limit(10)),
    IntervalLimit::upper(false, LimitValue::Limit(10))
  );
  assert_ne!(
    IntervalLimit::lower(false, LimitValue::Limit(10)),
    IntervalLimit::upper(true, LimitValue::Limit(10))
  );
  assert_ne!(
    IntervalLimit::lower(true, LimitValue::Limit(10)),
    IntervalLimit::upper(true, LimitValue::Limit(10))
  );

  assert_ne!(
    IntervalLimit::upper(false, LimitValue::Limit(10)),
    IntervalLimit::lower(false, LimitValue::Limit(10))
  );
  assert_ne!(
    IntervalLimit::upper(true, LimitValue::Limit(10)),
    IntervalLimit::lower(false, LimitValue::Limit(10))
  );
  assert_ne!(
    IntervalLimit::upper(false, LimitValue::Limit(10)),
    IntervalLimit::lower(true, LimitValue::Limit(10))
  );
  assert_ne!(
    IntervalLimit::upper(true, LimitValue::Limit(10)),
    IntervalLimit::lower(true, LimitValue::Limit(10))
  );

  assert_eq!(
    IntervalLimit::new(false, false, LimitValue::Limit(1)),
    IntervalLimit::new(false, false, LimitValue::Limit(1))
  );

  assert_eq!(
    IntervalLimit::lower(false, LimitValue::Limit(10)),
    IntervalLimit::lower(false, LimitValue::Limit(10))
  );
  assert_eq!(
    IntervalLimit::new(false, true, LimitValue::Limit(10)),
    IntervalLimit::lower(false, LimitValue::Limit(10))
  );
}

#[test]
fn test02_compare_to() {
  let lower_inf = IntervalLimit::<i32>::lower(false, LimitValue::Limitless);
  let upper_inf = IntervalLimit::<i32>::upper(false, LimitValue::Limitless);
  let lower_open2 = IntervalLimit::lower(false, LimitValue::Limit(2));
  let lower_close2 = IntervalLimit::lower(true, LimitValue::Limit(2));
  let lower_open3 = IntervalLimit::lower(false, LimitValue::Limit(3));
  let lower_close3 = IntervalLimit::lower(true, LimitValue::Limit(3));
  let upper_open5 = IntervalLimit::upper(false, LimitValue::Limit(5));
  let upper_close5 = IntervalLimit::upper(true, LimitValue::Limit(5));
  let upper_open6 = IntervalLimit::upper(false, LimitValue::Limit(6));
  let upper_close6 = IntervalLimit::upper(true, LimitValue::Limit(6));

  assert!(lower_inf < upper_inf);
  assert!(upper_inf > lower_inf);

  assert!(lower_inf < lower_open3);
  assert!(lower_inf < lower_close3);
  assert!(lower_inf < upper_open5);
  assert!(lower_inf < upper_close5);
  assert!(upper_inf > lower_open3);
  assert!(upper_inf > lower_close3);
  assert!(upper_inf > upper_open5);
  assert!(upper_inf > upper_close5);

  assert!(lower_open3 > lower_inf);
  assert!(lower_close3 > lower_inf);
  assert!(upper_open5 > lower_inf);
  assert!(upper_close5 > lower_inf);
  assert!(lower_open3 < upper_inf);
  assert!(lower_close3 < upper_inf);
  assert!(upper_open5 < upper_inf);
  assert!(upper_close5 < upper_inf);

  // 有限比較
  assert!(lower_close2 == lower_close2);
  assert!(lower_close2 < lower_open2);
  assert!(lower_close2 < lower_close3);
  assert!(lower_close2 < lower_open3);
  assert!(lower_close2 < upper_close5);
  assert!(lower_close2 < upper_open5);
  assert!(lower_close2 < upper_close6);
  assert!(lower_close2 < upper_open6);

  assert!(lower_open2 > lower_close2);
  assert!(lower_open2 == lower_open2);
  assert!(lower_open2 < lower_close3);
  assert!(lower_open2 < lower_open3);
  assert!(lower_open2 < upper_close5);
  assert!(lower_open2 < upper_open5);
  assert!(lower_open2 < upper_close6);
  assert!(lower_open2 < upper_open6);

  assert!(lower_close3 > lower_close2);
  assert!(lower_close3 > lower_open2);
  assert!(lower_close3 == lower_close3);
  assert!(lower_close3 < lower_open3);
  assert!(lower_close3 < upper_close5);
  assert!(lower_close3 < upper_open5);
  assert!(lower_close3 < upper_close6);
  assert!(lower_close3 < upper_open6);

  assert!(lower_open3 > lower_close2);
  assert!(lower_open3 > lower_open2);
  assert!(lower_open3 > lower_close3);
  assert!(lower_open3 == lower_open3);
  assert!(lower_open3 < upper_close5);
  assert!(lower_open3 < upper_open5);
  assert!(lower_open3 < upper_close6);
  assert!(lower_open3 < upper_open6);

  assert!(upper_close5 > lower_close2);
  assert!(upper_close5 > lower_open2);
  assert!(upper_close5 > lower_close3);
  assert!(upper_close5 > lower_open3);
  assert!(upper_close5 == upper_close5);
  assert!(upper_close5 > upper_open5);
  assert!(upper_close5 < upper_close6);
  assert!(upper_close5 < upper_open6);

  assert!(upper_open5 > lower_close2);
  assert!(upper_open5 > lower_open2);
  assert!(upper_open5 > lower_close3);
  assert!(upper_open5 > lower_open3);
  assert!(upper_open5 < upper_close5);
  assert!(upper_open5 == upper_open5);
  assert!(upper_open5 < upper_close6);
  assert!(upper_open5 < upper_open6);

  assert!(upper_close6 > lower_close2);
  assert!(upper_close6 > lower_open2);
  assert!(upper_close6 > lower_close3);
  assert!(upper_close6 > lower_open3);
  assert!(upper_close6 > upper_close5);
  assert!(upper_close6 > upper_open5);
  assert!(upper_close6 == upper_close6);
  assert!(upper_close6 > upper_open6);

  assert!(upper_open6 > lower_close2);
  assert!(upper_open6 > lower_open2);
  assert!(upper_open6 > lower_close3);
  assert!(upper_open6 > lower_open3);
  assert!(upper_open6 > upper_close5);
  assert!(upper_open6 > upper_open5);
  assert!(upper_open6 < upper_close6);
  assert!(upper_open6 == upper_open6);
}

#[test]
fn test03_sort() {
  let mut list: Vec<IntervalLimit<i32>> = vec![];
  list.push(IntervalLimit::upper(false, LimitValue::Limitless));
  list.push(IntervalLimit::upper(true, LimitValue::Limitless));
  list.push(IntervalLimit::lower(false, LimitValue::Limitless));
  list.push(IntervalLimit::lower(true, LimitValue::Limitless));
  list.push(IntervalLimit::lower(true, LimitValue::Limit(1)));
  list.push(IntervalLimit::lower(false, LimitValue::Limit(1)));
  list.push(IntervalLimit::lower(true, LimitValue::Limit(5)));
  list.push(IntervalLimit::lower(false, LimitValue::Limit(5)));
  list.push(IntervalLimit::upper(true, LimitValue::Limit(1)));
  list.push(IntervalLimit::upper(false, LimitValue::Limit(1)));
  list.push(IntervalLimit::upper(true, LimitValue::Limit(5)));
  list.push(IntervalLimit::upper(false, LimitValue::Limit(5)));

  let mut rng = rand::thread_rng();
  list.shuffle(&mut rng);
  list.sort_by(|a, b| a.partial_cmp(b).unwrap());

  assert_eq!(
    list.get(0).unwrap(),
    &IntervalLimit::lower(false, LimitValue::Limitless)
  );
  assert_eq!(
    list.get(1).unwrap(),
    &IntervalLimit::lower(false, LimitValue::Limitless)
  );

  assert_eq!(
    list.get(2).unwrap(),
    &IntervalLimit::lower(true, LimitValue::Limit(1))
  );
  assert_eq!(
    list.get(3).unwrap(),
    &IntervalLimit::lower(false, LimitValue::Limit(1))
  );
  assert_eq!(
    list.get(4).unwrap(),
    &IntervalLimit::upper(false, LimitValue::Limit(1))
  );
  assert_eq!(
    list.get(5).unwrap(),
    &IntervalLimit::upper(true, LimitValue::Limit(1))
  );
  assert_eq!(
    list.get(6).unwrap(),
    &IntervalLimit::lower(true, LimitValue::Limit(5))
  );
  assert_eq!(
    list.get(7).unwrap(),
    &IntervalLimit::lower(false, LimitValue::Limit(5))
  );
  assert_eq!(
    list.get(8).unwrap(),
    &IntervalLimit::upper(false, LimitValue::Limit(5))
  );
  assert_eq!(
    list.get(9).unwrap(),
    &IntervalLimit::upper(true, LimitValue::Limit(5))
  );

  assert_eq!(
    list.get(10).unwrap(),
    &IntervalLimit::upper(false, LimitValue::Limitless)
  );
  assert_eq!(
    list.get(11).unwrap(),
    &IntervalLimit::upper(false, LimitValue::Limitless)
  );
}
