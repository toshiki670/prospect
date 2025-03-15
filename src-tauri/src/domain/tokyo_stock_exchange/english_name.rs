#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct EnglishName(Option<String>);

impl core::ops::Deref for EnglishName {
    type Target = Option<String>;

    fn deref(&'_ self) -> &'_ Self::Target {
        &self.0
    }
}

impl From<Option<String>> for EnglishName {
    fn from(value: Option<String>) -> Self {
        Self(value)
    }
}
