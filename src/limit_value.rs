use crate::Error;
use std::cmp::Ordering;
use std::fmt::{Display, Formatter};
use crate::limit_value::LimitValue::Limitless;

#[derive(Debug, Clone, Eq, Ord)]
pub enum LimitValue<T> {
    Limit(T),
    Limitless,
}

impl<T: PartialEq> PartialEq for LimitValue<T> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (LimitValue::Limit(a), LimitValue::Limit(b)) => a == b,
            _ => false,
        }
    }
}

impl<T: PartialOrd> PartialOrd for LimitValue<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (LimitValue::Limit(a), LimitValue::Limit(b)) => a.partial_cmp(b),
            (_, LimitValue::Limitless) => Some(Ordering::Less),
            _ => Some(Ordering::Greater),
        }
    }
}

impl<T> LimitValue<T> {

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

impl<T: Display> Display for LimitValue<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            LimitValue::Limit(a) => write!(f, "Limit({})", a),
            LimitValue::Limitless => write!(f, "Limitless"),
        }
    }
}