use std::ops::SubAssign;

/// Represents the number of times a task will be fully ran.
///
/// This can be a fine grained value or the default of Infinity.
/// Infinity is equivalent to Custom(0).
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
pub enum Shot {
    Infinity,
    Custom(usize),
}

impl From<usize> for Shot {
    fn from(value: usize) -> Self {
        match value {
            0 => Self::Infinity,
            other => Self::Custom(other),
        }
    }
}

impl From<Shot> for usize {
    fn from(value: Shot) -> usize {
        match value {
            Shot::Infinity | Shot::Custom(0) => 0,
            Shot::Custom(other) => other,
        }
    }
}

impl SubAssign<usize> for Shot {
    fn sub_assign(&mut self, rhs: usize) {
        match self {
            Self::Infinity | Self::Custom(0) => (),
            Self::Custom(val) => *val -= rhs,
        }
    }
}
