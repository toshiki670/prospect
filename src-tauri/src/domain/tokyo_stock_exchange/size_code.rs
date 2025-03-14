#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct SizeCode(String);

impl core::ops::Deref for SizeCode {
    type Target = String;

    fn deref(&'_ self) -> &'_ Self::Target {
        &self.0
    }
}
