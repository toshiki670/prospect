#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct Section(String);

impl core::ops::Deref for Section {
    type Target = String;

    fn deref(&'_ self) -> &'_ Self::Target {
        &self.0
    }
}
