use anyhow::Context as _;
use async_trait::async_trait;
use calamine::{Reader, Xlsx, open_workbook};
use std::path::PathBuf;

use crate::domain::tokyo_stock_exchange::{
    english_name::EnglishName, japanese_name::JapaneseName, local_code::LocalCode,
    repository::TokyoStockExchangeAttributesQueryRepository, section::Section,
    sector17_code::Sector17Code, sector33_code::Sector33Code, size_code::SizeCode,
    tokyo_stock_exchange::TokyoStockExchangeAttributes,
};

pub struct TokyoStockExchangeFileRepositoryImpl {
    japanese_file: PathBuf,
    english_file: PathBuf,
}

#[derive(Debug)]
struct ExcelData {
    local_code: String,
    name: String,
    section: String,
    sector33_code: Option<i16>,
    sector17_code: Option<i16>,
    size_code: Option<i16>,
}

impl TokyoStockExchangeFileRepositoryImpl {
    pub fn new(japanese_file: PathBuf, english_file: PathBuf) -> Self {
        Self {
            japanese_file,
            english_file,
        }
    }

    fn convert_code(code: String) -> anyhow::Result<Option<i16>> {
        if code.contains("-") {
            return Ok(None);
        }

        let value = code
            .parse::<i16>()
            .with_context(|| format!("Failed to parse code: {}", code))?;

        Ok(Some(value))
    }

    fn read_excel(file_path: &PathBuf) -> anyhow::Result<Vec<ExcelData>> {
        let mut workbook: Xlsx<_> = open_workbook(file_path)
            .with_context(|| format!("Failed to open Japanese file: {}", file_path.display()))?;

        let range = workbook
            .worksheet_range("Sheet1")
            .with_context(|| "Failed to read Sheet1")?;

        let mut results = Vec::new();

        for (index, row) in range.rows().skip(1).enumerate() {
            let sector33_code = Self::convert_code(row[4].to_string()).with_context(|| {
                format!(
                    "Failed to read: row={}, column={}, data={}",
                    index, 4, row[4]
                )
            })?;
            let sector17_code = Self::convert_code(row[6].to_string()).with_context(|| {
                format!(
                    "Failed to read: row={}, column={}, data={}",
                    index, 6, row[6]
                )
            })?;
            let size_code = Self::convert_code(row[8].to_string()).with_context(|| {
                format!(
                    "Failed to read: row={}, column={}, data={}",
                    index, 8, row[8]
                )
            })?;

            let data = ExcelData {
                local_code: row[1].to_string(),
                name: row[2].to_string(),
                section: row[3].to_string(),
                sector33_code,
                sector17_code,
                size_code,
            };
            results.push(data);
        }

        Ok(results)
    }

    fn read_japanese_excel(&self) -> anyhow::Result<Vec<ExcelData>> {
        Self::read_excel(&self.japanese_file)
    }

    fn read_english_excel(&self) -> anyhow::Result<Vec<ExcelData>> {
        Self::read_excel(&self.english_file)
    }

    fn merge_excel_data(
        &self,
        ja_data: Vec<ExcelData>,
        en_data: Vec<ExcelData>,
    ) -> anyhow::Result<Vec<TokyoStockExchangeAttributes>> {
        let mut results = Vec::new();

        // ローカルコードをキーにしてマージ
        for ja in ja_data {
            if let Some(en) = en_data.iter().find(|en| en.local_code == ja.local_code) {
                let attributes = TokyoStockExchangeAttributes {
                    local_code: LocalCode::try_from(ja.local_code)?,
                    english_name: EnglishName::from(en.name.clone()),
                    japanese_name: JapaneseName::from(ja.name),
                    section: Section::try_from(ja.section)?,
                    sector33_code: Sector33Code::try_from(ja.sector33_code)?,
                    sector17_code: Sector17Code::try_from(ja.sector17_code)?,
                    size_code: SizeCode::try_from(ja.size_code)?,
                };
                results.push(attributes);
            }
        }

        Ok(results)
    }
}

#[async_trait]
impl TokyoStockExchangeAttributesQueryRepository for TokyoStockExchangeFileRepositoryImpl {
    async fn find_all(&self) -> anyhow::Result<Vec<TokyoStockExchangeAttributes>> {
        let ja_data = self.read_japanese_excel()?;
        let en_data = self.read_english_excel()?;
        let results = self.merge_excel_data(ja_data, en_data)?;

        Ok(results)
    }
}
