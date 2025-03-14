#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct Id(u32);

impl core::ops::Deref for Id {
    type Target = u32;

    fn deref(&'_ self) -> &'_ Self::Target {
        &self.0
    }
}
