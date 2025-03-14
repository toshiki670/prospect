use chrono::prelude::*;

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct UpdatedAt(DateTime<Local>);

impl core::ops::Deref for UpdatedAt {
    type Target = DateTime<Local>;

    fn deref(&'_ self) -> &'_ Self::Target {
        &self.0
    }
}
