use chrono::prelude::*;

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct UpdatedAt(DateTime<Local>);

impl core::ops::Deref for UpdatedAt {
    type Target = DateTime<Local>;

    fn deref(&'_ self) -> &'_ Self::Target {
        &self.0
    }
}

impl From<DateTime<Utc>> for UpdatedAt {
    fn from(value: DateTime<Utc>) -> Self {
        Self(value.with_timezone(&Local))
    }
}
