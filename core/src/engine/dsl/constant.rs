use std::fmt::Display;

#[derive(Debug, Copy, Clone)]
pub enum Constant {
    Amount(f64),
    Count(u128),
}

impl Constant {
    pub fn as_f64(&self) -> f64 {
        match self {
            Self::Amount(a) => *a,
            Self::Count(c) => *c as f64,
        }
    }
}

impl PartialEq for Constant {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Amount(a), Self::Amount(b)) => a == b,
            (Self::Count(a), Self::Count(b)) => a == b,
            (Self::Amount(a), Self::Count(b)) => *a == *b as f64,
            (Self::Count(a), Self::Amount(b)) => *a as f64 == *b,
        }
    }
}

impl PartialOrd for Constant {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (Self::Amount(a), Self::Amount(b)) => a.partial_cmp(b),
            (Self::Count(a), Self::Count(b)) => a.partial_cmp(b),
            (Self::Amount(a), Self::Count(b)) => a.partial_cmp(&(*b as f64)),
            (Self::Count(a), Self::Amount(b)) => (*a as f64).partial_cmp(b),
        }
    }
}

impl Display for Constant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Amount(a) => write!(f, "{a}"),
            Self::Count(a) => write!(f, "{a}"),
        }
    }
}

impl From<f64> for Constant {
    fn from(a: f64) -> Self {
        Self::Amount(a)
    }
}

impl From<u128> for Constant {
    fn from(a: u128) -> Self {
        Self::Count(a)
    }
}

impl From<usize> for Constant {
    fn from(a: usize) -> Self {
        Self::Count(a as u128)
    }
}
