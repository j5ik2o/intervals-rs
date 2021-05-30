use std::cmp::Ordering;
use std::fmt::{Display, Formatter, Debug};

use crate::Error;
use crate::limit_value::LimitValue::Limitless;

#[derive(Debug, Clone)]
pub enum LimitValue<T: Debug> {
    Limit(T),
    Limitless,
}

impl<T: Debug + Clone + Default> Default for LimitValue<T> {
    fn default() -> Self {
        LimitValue::Limitless
    }
}

impl<T: Debug + PartialEq> PartialEq for LimitValue<T> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (LimitValue::Limitless, LimitValue::Limitless) => true,
            (LimitValue::Limit(value), LimitValue::Limit(other_value)) =>
                value == other_value,
            _ => false,
        }
    }
}

impl<T: Debug + PartialOrd> PartialOrd for LimitValue<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (LimitValue::Limitless, LimitValue::Limitless) => Some(Ordering::Equal),
            (LimitValue::Limit(_), LimitValue::Limitless) => Some(Ordering::Greater),
            (LimitValue::Limitless, LimitValue::Limit(_)) => Some(Ordering::Less),
            (LimitValue::Limit(value), LimitValue::Limit(other_value)) =>
                value.partial_cmp(other_value),
        }
    }
}

impl<T: Debug> LimitValue<T> {
    pub fn from_limit_value(value: Option<T>) -> Self {
        match value {
            None => LimitValue::Limitless,
            Some(v) => LimitValue::Limit(v),
        }
    }

    pub fn is_limitless(&self) -> bool {
        match self {
            LimitValue::Limitless => true,
            _ => false,
        }
    }

    pub fn to_value(&self) -> Result<&T, Error> {
        match self {
            LimitValue::Limit(a) => Ok(a),
            LimitValue::Limitless => Err(Error::NotFoundError),
        }
    }

    pub fn to_value_or<'a, TF>(&'a self, default: TF) -> &T
        where
            TF: Fn() -> &'a T,
    {
        match self {
            LimitValue::Limit(a) => a,
            LimitValue::Limitless => default(),
        }
    }
}

impl<T: Display + Debug> Display for LimitValue<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            LimitValue::Limit(a) => write!(f, "Limit({})", a),
            LimitValue::Limitless => write!(f, "Limitless"),
        }
    }
}

#[cfg(test)]
mod tests {
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
}