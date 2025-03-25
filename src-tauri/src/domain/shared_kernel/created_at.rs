use chrono::prelude::*;

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct CreatedAt(DateTime<Local>);

impl core::ops::Deref for CreatedAt {
    type Target = DateTime<Local>;

    fn deref(&'_ self) -> &'_ Self::Target {
        &self.0
    }
}

impl From<DateTime<Utc>> for CreatedAt {
    fn from(value: DateTime<Utc>) -> Self {
        Self(value.with_timezone(&Local))
    }
}
