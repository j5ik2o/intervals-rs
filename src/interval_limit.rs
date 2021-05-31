use std::cmp::Ordering;
use std::fmt::{Display, Formatter, Debug};

use crate::LimitValue;

#[derive(Debug, Clone)]
pub struct IntervalLimit<T: Debug + Clone + PartialOrd + PartialEq> {
  closed: bool,
  lower: bool,
  value: LimitValue<T>,
}

impl<T: Debug + Clone + PartialEq + PartialOrd> PartialEq for IntervalLimit<T> {
  fn eq(&self, other: &Self) -> bool {
    self.partial_cmp(other) == Some(Ordering::Equal)
  }
}

impl<T: Debug + Clone + PartialEq + PartialOrd> PartialOrd for IntervalLimit<T> {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    if self.value.is_limitless() && other.value.is_limitless() {
      if self.lower == other.lower {
        Some(Ordering::Equal)
      } else {
        self.lower_to_ordering(Some(Ordering::Less), Some(Ordering::Greater))
      }
    } else if self.value.is_limitless() {
      self.lower_to_ordering(Some(Ordering::Less), Some(Ordering::Greater))
    } else if other.value.is_limitless() {
      other.lower_to_ordering(Some(Ordering::Greater), Some(Ordering::Less))
    } else if self.value == other.value {
      if self.lower && other.lower {
        if self.closed ^ other.closed {
          self.closed_to_ordering(Some(Ordering::Less), Some(Ordering::Greater))
        } else {
          Some(Ordering::Equal)
        }
      } else if !self.lower && !other.lower {
        if self.closed ^ other.closed {
          self.closed_to_ordering(Some(Ordering::Greater), Some(Ordering::Less))
        } else {
          Some(Ordering::Equal)
        }
      } else {
        self.lower_to_ordering(Some(Ordering::Less), Some(Ordering::Greater))
      }
    } else {
      self.value.partial_cmp(&other.value)
    }
  }
}

impl<T: Debug + Clone + PartialEq + PartialOrd> IntervalLimit<T> {
  pub fn is_closed(&self) -> bool {
    self.closed
  }

  pub fn is_lower(&self) -> bool {
    self.lower
  }

  pub fn get_value(&self) -> &LimitValue<T> {
    &self.value
  }

  pub fn new(closed: bool, lower: bool, value: LimitValue<T>) -> Self {
    Self {
      closed: if value.is_limitless() { false } else { closed },
      lower,
      value,
    }
  }

  pub fn lower(closed: bool, value: LimitValue<T>) -> Self {
    Self::new(closed, true, value)
  }

  pub fn upper(closed: bool, value: LimitValue<T>) -> Self {
    Self::new(closed, false, value)
  }

  fn lower_to_ordering<A>(&self, t: A, f: A) -> A {
    if self.lower {
      t
    } else {
      f
    }
  }

  fn closed_to_ordering<A>(&self, t: A, f: A) -> A {
    if self.closed {
      t
    } else {
      f
    }
  }

  pub fn infinity(&self) -> bool {
    matches!(self.value, LimitValue::Limitless)
  }

  pub fn is_open(&self) -> bool {
    !self.closed
  }

  pub fn is_upper(&self) -> bool {
    !self.lower
  }
}

impl<T: Debug + Display + Clone + PartialEq + PartialOrd> Display for IntervalLimit<T> {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(
      f,
      "IntervalLimit({}, {}, {})",
      self.closed, self.lower, self.value
    )
  }
}

#[cfg(test)]
mod tests {
  use crate::interval_limit::IntervalLimit;
  use crate::LimitValue;
  use rand::seq::SliceRandom;

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
}
