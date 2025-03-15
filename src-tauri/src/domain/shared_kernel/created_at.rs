use chrono::prelude::*;

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct CreatedAt(DateTime<Local>);

impl core::ops::Deref for CreatedAt {
    type Target = DateTime<Local>;

    fn deref(&'_ self) -> &'_ Self::Target {
        &self.0
    }
}
