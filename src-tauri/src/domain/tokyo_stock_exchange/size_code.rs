#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct SizeCode(Option<i16>);

impl core::ops::Deref for SizeCode {
    type Target = Option<i16>;

    fn deref(&'_ self) -> &'_ Self::Target {
        &self.0
    }
}

impl TryFrom<Option<i16>> for SizeCode {
    type Error = anyhow::Error;

    fn try_from(value: Option<i16>) -> Result<Self, anyhow::Error> {
        let value = Self(value);

        Ok(value)
    }
}
