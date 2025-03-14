#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct JapaneseName(String);

impl core::ops::Deref for JapaneseName {
    type Target = String;

    fn deref(&'_ self) -> &'_ Self::Target {
        &self.0
    }
}
