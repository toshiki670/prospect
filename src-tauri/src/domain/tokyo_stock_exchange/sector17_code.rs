#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct Sector17Code(String);

impl core::ops::Deref for Sector17Code {
    type Target = String;

    fn deref(&'_ self) -> &'_ Self::Target {
        &self.0
    }
}
