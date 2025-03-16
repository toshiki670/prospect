#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct Section(String);

impl core::ops::Deref for Section {
    type Target = String;

    fn deref(&'_ self) -> &'_ Self::Target {
        &self.0
    }
}

impl TryFrom<String> for Section {
    type Error = anyhow::Error;

    fn try_from(value: String) -> Result<Self, anyhow::Error> {
        Ok(Self(value))
    }
}
