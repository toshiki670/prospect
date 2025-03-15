#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct EnglishName(String);

impl core::ops::Deref for EnglishName {
    type Target = String;

    fn deref(&'_ self) -> &'_ Self::Target {
        &self.0
    }
}
