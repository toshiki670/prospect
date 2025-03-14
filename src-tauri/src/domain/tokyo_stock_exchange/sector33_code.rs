#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct Sector33Code(String);

impl core::ops::Deref for Sector33Code {
    type Target = String;

    fn deref(&'_ self) -> &'_ Self::Target {
        &self.0
    }
}
