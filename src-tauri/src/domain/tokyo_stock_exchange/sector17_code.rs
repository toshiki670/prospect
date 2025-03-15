use crate::domain::DomainError;

/// 17セクターコード
///
/// 17セクターコードは、東証17セクターのコードです。
///
/// 以下のコードが存在します。
///
/// | Code | 17業種区分                 |
/// |------|---------------------------|
/// | -    | -                         |
/// | 1    | 食品                      |
/// | 2    | エネルギー資源            |
/// | 3    | 建設・資材                |
/// | 4    | 素材・化学                |
/// | 5    | 医薬品                    |
/// | 6    | 自動車・輸送機            |
/// | 7    | 鉄鋼・非鉄                |
/// | 8    | 機械                      |
/// | 9    | 電機・精密                |
/// | 10   | 情報通信・サービスその他  |
/// | 11   | 電力・ガス                |
/// | 12   | 運輸・物流                |
/// | 13   | 商社・卸売                |
/// | 14   | 小売                      |
/// | 15   | 銀行                      |
/// | 16   | 金融（除く銀行）          |
/// | 17   | 不動産                    |
///
#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct Sector17Code(Option<i16>);

impl core::ops::Deref for Sector17Code {
    type Target = Option<i16>;

    fn deref(&'_ self) -> &'_ Self::Target {
        &self.0
    }
}

impl TryFrom<Option<i16>> for Sector17Code {
    type Error = anyhow::Error;

    fn try_from(value: Option<i16>) -> Result<Self, anyhow::Error> {
        let value = Self(value);

        value.validate()?;

        Ok(value)
    }
}

impl Sector17Code {
    pub fn validate(&self) -> anyhow::Result<()> {
        match self.0 {
            Some(code) => match code {
                1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 => Ok(()),
                _ => Err(DomainError::ValidationError("Invalid sector17 code".to_string()).into()),
            },
            None => Ok(()),
        }
    }
}

impl std::fmt::Display for Sector17Code {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.0 {
            Some(1) => write!(f, "食品"),
            Some(2) => write!(f, "エネルギー資源"),
            Some(3) => write!(f, "建設・資材"),
            Some(4) => write!(f, "素材・化学"),
            Some(5) => write!(f, "医薬品"),
            Some(6) => write!(f, "自動車・輸送機"),
            Some(7) => write!(f, "鉄鋼・非鉄"),
            Some(8) => write!(f, "機械"),
            Some(9) => write!(f, "電機・精密"),
            Some(10) => write!(f, "情報通信・サービスその他"),
            Some(11) => write!(f, "電力・ガス"),
            Some(12) => write!(f, "運輸・物流"),
            Some(13) => write!(f, "商社・卸売"),
            Some(14) => write!(f, "小売"),
            Some(15) => write!(f, "銀行"),
            Some(16) => write!(f, "金融（除く銀行）"),
            Some(17) => write!(f, "不動産"),
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
    #[case::none(None)] // -
    #[case::foods(Some(1))] // 食品
    #[case::energy(Some(2))] // エネルギー資源
    #[case::construction(Some(3))] // 建設・資材
    #[case::materials(Some(4))] // 素材・化学
    #[case::pharmaceutical(Some(5))] // 医薬品
    #[case::automobile(Some(6))] // 自動車・輸送機
    #[case::steel(Some(7))] // 鉄鋼・非鉄
    #[case::machinery(Some(8))] // 機械
    #[case::electric(Some(9))] // 電機・精密
    #[case::information(Some(10))] // 情報通信・サービスその他
    #[case::electric_power(Some(11))] // 電力・ガス
    #[case::transportation(Some(12))] // 運輸・物流
    #[case::wholesale(Some(13))] // 商社・卸売
    #[case::retail(Some(14))] // 小売
    #[case::banks(Some(15))] // 銀行
    #[case::financial(Some(16))] // 金融（除く銀行）
    #[case::real_estate(Some(17))] // 不動産
    fn test_validate_sector17_code_success(#[case] code: Option<i16>) {
        let sector17_code = Sector17Code::try_from(code);
        assert!(sector17_code.is_ok());
    }

    #[rstest]
    #[case::negative(Some(0), "Invalid sector17 code")]
    #[case::invalid_code(Some(1000), "Invalid sector17 code")] // 1000は存在しない
    #[case::too_large(Some(9999), "Invalid sector17 code")] // 9999は存在しない
    #[case::not_defined(Some(3999), "Invalid sector17 code")] // 3999は定義されていない
    fn test_validate_sector17_code_error(#[case] code: Option<i16>, #[case] expected_error: &str) {
        let sector17_code = Sector17Code::try_from(code);

        assert!(sector17_code.is_err());

        let error: DomainError = sector17_code.unwrap_err().try_into().unwrap();
        let error_message: String = error.into();
        assert_eq!(error_message, expected_error);
    }

    #[rstest]
    #[case::none(None, None)]
    #[case::foods(Some(1), Some(1))] // 食品
    fn test_deref(#[case] input: Option<i16>, #[case] expected: Option<i16>) {
        let sector17_code = Sector17Code::try_from(input).unwrap();
        assert_eq!(*sector17_code, expected);
    }

    #[rstest]
    #[case::none(None, "-")]
    #[case::foods(Some(1), "食品")]
    #[case::energy(Some(2), "エネルギー資源")]
    #[case::construction(Some(3), "建設・資材")]
    #[case::materials(Some(4), "素材・化学")]
    #[case::pharmaceutical(Some(5), "医薬品")]
    #[case::automobile(Some(6), "自動車・輸送機")]
    #[case::steel(Some(7), "鉄鋼・非鉄")]
    #[case::machinery(Some(8), "機械")]
    #[case::electric(Some(9), "電機・精密")]
    #[case::information(Some(10), "情報通信・サービスその他")]
    #[case::electric_power(Some(11), "電力・ガス")]
    #[case::transportation(Some(12), "運輸・物流")]
    #[case::wholesale(Some(13), "商社・卸売")]
    #[case::retail(Some(14), "小売")]
    #[case::banks(Some(15), "銀行")]
    #[case::financial(Some(16), "金融（除く銀行）")]
    #[case::real_estate(Some(17), "不動産")]
    fn test_display(#[case] input: Option<i16>, #[case] expected: &str) {
        let sector17_code = Sector17Code::try_from(input).unwrap();
        assert_eq!(format!("{}", sector17_code), expected);
    }
}
