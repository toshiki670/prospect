#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct SizeCode(i16);

impl core::ops::Deref for SizeCode {
    type Target = i16;

    fn deref(&'_ self) -> &'_ Self::Target {
        &self.0
    }
}
