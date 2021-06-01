mod errors;

mod interval;
mod interval_limit;
mod interval_seq;
mod limit_value;

#[cfg(test)]
mod interval_limit_test;
#[cfg(test)]
mod interval_seq_test;
#[cfg(test)]
mod interval_test;
#[cfg(test)]
mod limit_value_test;

pub use crate::errors::Error;
pub use crate::limit_value::LimitValue;
pub use crate::interval_limit::IntervalLimit;
pub use crate::interval::Interval;
pub use crate::interval_seq::IntervalSeq;
use std::cmp::Ordering;

pub fn to_ordering(n: i8) -> Ordering {
    match n {
        -1 => Ordering::Less,
        0 => Ordering::Equal,
        1 => Ordering::Greater,
        _ => panic!(),
    }
}