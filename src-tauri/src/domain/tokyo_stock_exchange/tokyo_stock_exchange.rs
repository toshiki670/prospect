use crate::domain::shared_kernel::{created_at::CreatedAt, id::Id, updated_at::UpdatedAt};

use super::{
    english_name::EnglishName, japanese_name::JapaneseName, local_code::LocalCode,
    section::Section, sector17_code::Sector17Code, sector33_code::Sector33Code,
    size_code::SizeCode,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TokyoStockExchange {
    pub id: Option<Id>,
    pub local_code: LocalCode,
    pub english_name: EnglishName,
    pub japanese_name: JapaneseName,
    pub section: Section,
    pub sector33_code: Sector33Code,
    pub sector17_code: Sector17Code,
    pub size_code: SizeCode,
    pub created_at: CreatedAt,
    pub updated_at: UpdatedAt,
}
