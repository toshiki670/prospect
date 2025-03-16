use crate::domain::DomainError;

/// 33セクターコード
///
/// 33セクターコードは、東証33セクターのコードです。
///
/// 以下のコードが存在します。
///
/// | 指数名               | 業種コード | 業種                   | 会社数 |
/// |----------------------|------------|------------------------|--------|
/// | null                 | null       | null                   | null   |
/// | 鉱業                 | 0050       | 水産・農林業             | 7      |
/// | 鉱業                 | 1050       | 鉱業                    | 6      |
/// | 建設業               | 2050       | 建設業                  | 101    |
/// | 製造業               | 3050       | 食料品                  | 82     |
/// | 製造業               | 3100       | 繊維製品                | 40     |
/// | 製造業               | 3150       | パルプ・紙              | 12     |
/// | 製造業               | 3200       | 化学                   | 145    |
/// | 製造業               | 3250       | 医薬品                  | 38     |
/// | 製造業               | 3300       | 石油・石炭製品           | 9      |
/// | 製造業               | 3350       | ゴム製品                | 11     |
/// | 製造業               | 3400       | ガラス・土石製品         | 33     |
/// | 製造業               | 3450       | 鉄鋼                   | 31     |
/// | 製造業               | 3500       | 非鉄金属                | 24     |
/// | 製造業               | 3550       | 金属製品                | 41     |
/// | 製造業               | 3600       | 機械                   | 140    |
/// | 製造業               | 3650       | 電気機器                | 157    |
/// | 製造業               | 3700       | 輸送用機器              | 62     |
/// | 製造業               | 3750       | 精密機器                | 33     |
/// | 製造業               | 3800       | その他製品              | 52     |
/// | 電気・ガス業          | 4050       | 電気・ガス業            | 22     |
/// | 運輸・情報通信業      | 5050       | 陸運業                  | 43     |
/// | 運輸・情報通信業      | 5100       | 海運業                  | 8      |
/// | 運輸・情報通信業      | 5150       | 空運業                  | 3      |
/// | 運輸・情報通信業      | 5200       | 倉庫・運輸関連業         | 24     |
/// | 運輸・情報通信業      | 5250       | 情報・通信業             | 227    |
/// | 商業                | 6050       | 卸売業                  | 179    |
/// | 商業                | 6100       | 小売業                  | 202    |
/// | 金融・保険業         | 7050       | 銀行業                  | 82     |
/// | 金融・保険業         | 7100       | 証券・商品先物取引業      | 22     |
/// | 金融・保険業         | 7150       | 保険業                  | 8      |
/// | 金融・保険業         | 7200       | その他金融業             | 27     |
/// | 不動産業             | 8050       | 不動産業               | 70     |
/// | サービス業           | 9050       | サービス業              | 22     |
///
#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct Sector33Code(Option<i16>);

impl core::ops::Deref for Sector33Code {
    type Target = Option<i16>;

    fn deref(&'_ self) -> &'_ Self::Target {
        &self.0
    }
}

impl TryFrom<Option<i16>> for Sector33Code {
    type Error = anyhow::Error;

    fn try_from(value: Option<i16>) -> Result<Self, anyhow::Error> {
        let value = Self(value);

        value.validate()?;

        Ok(value)
    }
}

impl Sector33Code {
    pub fn validate(&self) -> anyhow::Result<()> {
        match self.0 {
            Some(code) => match code {
                50 | 1050 | 2050 | 3050 | 3100 | 3150 | 3200 | 3250 | 3300 | 3350 | 3400 | 3450
                | 3500 | 3550 | 3600 | 3650 | 3700 | 3750 | 3800 | 4050 | 5050 | 5100 | 5150
                | 5200 | 5250 | 6050 | 6100 | 7050 | 7100 | 7150 | 7200 | 8050 | 9050 => Ok(()),
                _ => Err(DomainError::ValidationError("Invalid sector33 code".to_string()).into()),
            },
            None => Ok(()),
        }
    }
}

impl std::fmt::Display for Sector33Code {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.0 {
            Some(50) => write!(f, "水産・農林業"),
            Some(1050) => write!(f, "鉱業"),
            Some(2050) => write!(f, "建設業"),
            Some(3050) => write!(f, "食料品"),
            Some(3100) => write!(f, "繊維製品"),
            Some(3150) => write!(f, "パルプ・紙"),
            Some(3200) => write!(f, "化学"),
            Some(3250) => write!(f, "医薬品"),
            Some(3300) => write!(f, "石油・石炭製品"),
            Some(3350) => write!(f, "ゴム製品"),
            Some(3400) => write!(f, "ガラス・土石製品"),
            Some(3450) => write!(f, "鉄鋼"),
            Some(3500) => write!(f, "非鉄金属"),
            Some(3550) => write!(f, "金属製品"),
            Some(3600) => write!(f, "機械"),
            Some(3650) => write!(f, "電気機器"),
            Some(3700) => write!(f, "輸送用機器"),
            Some(3750) => write!(f, "精密機器"),
            Some(3800) => write!(f, "その他製品"),
            Some(4050) => write!(f, "電気・ガス業"),
            Some(5050) => write!(f, "陸運業"),
            Some(5100) => write!(f, "海運業"),
            Some(5150) => write!(f, "空運業"),
            Some(5200) => write!(f, "倉庫・運輸関連業"),
            Some(5250) => write!(f, "情報・通信業"),
            Some(6050) => write!(f, "卸売業"),
            Some(6100) => write!(f, "小売業"),
            Some(7050) => write!(f, "銀行業"),
            Some(7100) => write!(f, "証券・商品先物取引業"),
            Some(7150) => write!(f, "保険業"),
            Some(7200) => write!(f, "その他金融業"),
            Some(8050) => write!(f, "不動産業"),
            Some(9050) => write!(f, "サービス業"),
            Some(_) => write!(f, "不明な業種"),
            None => write!(f, "-"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use rstest::*;

    #[rstest]
    #[case::none(None)] // null
    #[case::agriculture(Some(50))] // 水産・農林業
    #[case::mining(Some(1050))] // 鉱業
    #[case::construction(Some(2050))] // 建設業
    #[case::foods(Some(3050))] // 食料品
    #[case::textiles(Some(3100))] // 繊維製品
    #[case::pulp_paper(Some(3150))] // パルプ・紙
    #[case::chemicals(Some(3200))] // 化学
    #[case::pharmaceutical(Some(3250))] // 医薬品
    #[case::oil_coal(Some(3300))] // 石油・石炭製品
    #[case::rubber(Some(3350))] // ゴム製品
    #[case::glass_ceramics(Some(3400))] // ガラス・土石製品
    #[case::steel(Some(3450))] // 鉄鋼
    #[case::nonferrous_metals(Some(3500))] // 非鉄金属
    #[case::metal_products(Some(3550))] // 金属製品
    #[case::machinery(Some(3600))] // 機械
    #[case::electric_appliances(Some(3650))] // 電気機器
    #[case::transportation(Some(3700))] // 輸送用機器
    #[case::precision(Some(3750))] // 精密機器
    #[case::other_products(Some(3800))] // その他製品
    #[case::electric_power(Some(4050))] // 電気・ガス業
    #[case::land_transport(Some(5050))] // 陸運業
    #[case::marine_transport(Some(5100))] // 海運業
    #[case::air_transport(Some(5150))] // 空運業
    #[case::warehouse(Some(5200))] // 倉庫・運輸関連業
    #[case::information(Some(5250))] // 情報・通信業
    #[case::wholesale(Some(6050))] // 卸売業
    #[case::retail(Some(6100))] // 小売業
    #[case::banks(Some(7050))] // 銀行業
    #[case::securities(Some(7100))] // 証券・商品先物取引業
    #[case::insurance(Some(7150))] // 保険業
    #[case::other_financial(Some(7200))] // その他金融業
    #[case::real_estate(Some(8050))] // 不動産業
    #[case::services(Some(9050))] // サービス業
    fn test_validate_sector33_code_success(#[case] code: Option<i16>) {
        let sector33_code = Sector33Code::try_from(code);
        assert!(sector33_code.is_ok());
    }

    #[rstest]
    #[case::negative(Some(0), "Invalid sector33 code")]
    #[case::invalid_code(Some(1000), "Invalid sector33 code")] // 1000は存在しない
    #[case::too_large(Some(9999), "Invalid sector33 code")] // 9999は存在しない
    #[case::not_defined(Some(3999), "Invalid sector33 code")] // 3999は定義されていない
    fn test_validate_sector33_code_error(#[case] code: Option<i16>, #[case] expected_error: &str) {
        let sector33_code = Sector33Code::try_from(code);

        assert!(sector33_code.is_err());

        let error: DomainError = sector33_code.unwrap_err().try_into().unwrap();
        let error_message: String = error.into();
        assert_eq!(error_message, expected_error);
    }

    #[rstest]
    #[case::none(None, None)]
    #[case::foods(Some(3050), Some(3050))] // 食料品
    fn test_deref(#[case] input: Option<i16>, #[case] expected: Option<i16>) {
        let sector33_code = Sector33Code::try_from(input).unwrap();
        assert_eq!(*sector33_code, expected);
    }

    #[rstest]
    #[case::none(None, "-")]
    #[case::foods(Some(3050), "食料品")]
    #[case::textiles(Some(3100), "繊維製品")]
    #[case::pulp_paper(Some(3150), "パルプ・紙")]
    #[case::chemicals(Some(3200), "化学")]
    #[case::pharmaceutical(Some(3250), "医薬品")]
    #[case::oil_coal(Some(3300), "石油・石炭製品")]
    #[case::rubber(Some(3350), "ゴム製品")]
    #[case::glass_ceramics(Some(3400), "ガラス・土石製品")]
    #[case::steel(Some(3450), "鉄鋼")]
    #[case::nonferrous_metals(Some(3500), "非鉄金属")]
    #[case::metal_products(Some(3550), "金属製品")]
    #[case::machinery(Some(3600), "機械")]
    #[case::electric_appliances(Some(3650), "電気機器")]
    #[case::transportation(Some(3700), "輸送用機器")]
    #[case::precision(Some(3750), "精密機器")]
    #[case::other_products(Some(3800), "その他製品")]
    #[case::electric_power(Some(4050), "電気・ガス業")]
    #[case::land_transport(Some(5050), "陸運業")]
    #[case::marine_transport(Some(5100), "海運業")]
    #[case::air_transport(Some(5150), "空運業")]
    #[case::warehouse(Some(5200), "倉庫・運輸関連業")]
    #[case::information(Some(5250), "情報・通信業")]
    #[case::wholesale(Some(6050), "卸売業")]
    #[case::retail(Some(6100), "小売業")]
    #[case::banks(Some(7050), "銀行業")]
    #[case::securities(Some(7100), "証券・商品先物取引業")]
    #[case::insurance(Some(7150), "保険業")]
    #[case::other_financial(Some(7200), "その他金融業")]
    #[case::real_estate(Some(8050), "不動産業")]
    #[case::services(Some(9050), "サービス業")]
    fn test_display(#[case] input: Option<i16>, #[case] expected: &str) {
        let sector33_code = Sector33Code::try_from(input).unwrap();
        assert_eq!(format!("{}", sector33_code), expected);
    }
}
