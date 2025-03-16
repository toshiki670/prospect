use std::fmt;

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct Id(i32);

impl core::ops::Deref for Id {
    type Target = i32;

    fn deref(&'_ self) -> &'_ Self::Target {
        &self.0
    }
}

impl From<i32> for Id {
    fn from(value: i32) -> Self {
        Self(value)
    }
}

impl fmt::Display for Id {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
