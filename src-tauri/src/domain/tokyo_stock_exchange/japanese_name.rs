#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct JapaneseName(Option<String>);

impl core::ops::Deref for JapaneseName {
    type Target = Option<String>;

    fn deref(&'_ self) -> &'_ Self::Target {
        &self.0
    }
}

impl From<Option<String>> for JapaneseName {
    fn from(value: Option<String>) -> Self {
        Self(value)
    }
}
