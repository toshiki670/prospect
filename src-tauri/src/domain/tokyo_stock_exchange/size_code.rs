use crate::app_error::AppError;

/// Size Code
///
/// The following codes exist:
///
/// | Code | Category       |
/// |------|---------------|
/// | -    | -             |
/// | 1    | TOPIX Core30  |
/// | 2    | TOPIX Large70 |
/// | 4    | TOPIX Mid400  |
/// | 6    | TOPIX Small 1 |
/// | 7    | TOPIX Small 2 |
///
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

        value.validate()?;

        Ok(value)
    }
}

impl SizeCode {
    pub fn validate(&self) -> anyhow::Result<()> {
        match self.0 {
            Some(code) => match code {
                1 | 2 | 4 | 6 | 7 => Ok(()),
                _ => Err(AppError::ImvalidValidation("Invalid size code".to_string()).into()),
            },
            None => Ok(()),
        }
    }
}

impl std::fmt::Display for SizeCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.0 {
            Some(1) => write!(f, "TOPIX Core30"),
            Some(2) => write!(f, "TOPIX Large70"),
            Some(4) => write!(f, "TOPIX Mid400"),
            Some(6) => write!(f, "TOPIX Small 1"),
            Some(7) => write!(f, "TOPIX Small 2"),
            Some(_) => {
                unreachable!("Invalid size code: This code has been validated and should not exist")
            }
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
    #[case::none(None)] // No size code
    #[case::core30(Some(1))] // TOPIX Core30
    #[case::large70(Some(2))] // TOPIX Large70
    #[case::mid400(Some(4))] // TOPIX Mid400
    #[case::small1(Some(6))] // TOPIX Small 1
    #[case::small2(Some(7))] // TOPIX Small 2
    fn test_validate_size_code_success(#[case] code: Option<i16>) {
        let size_code = SizeCode::try_from(code);
        assert!(size_code.is_ok());
    }

    #[rstest]
    #[case::negative(Some(0), "Invalid size code")]
    #[case::invalid_code(Some(3), "Invalid size code")] // Code 3 does not exist
    #[case::too_large(Some(8), "Invalid size code")] // Code 8 does not exist
    fn test_validate_size_code_error(#[case] code: Option<i16>, #[case] expected_error: &str) {
        let size_code = SizeCode::try_from(code);

        assert!(size_code.is_err());

        let error: AppError = size_code.unwrap_err().into();
        let error_message: String = error.into();
        assert_eq!(error_message, expected_error);
    }

    #[rstest]
    #[case::none(None, None)]
    #[case::core30(Some(1), Some(1))] // TOPIX Core30
    fn test_deref(#[case] input: Option<i16>, #[case] expected: Option<i16>) {
        let size_code = SizeCode::try_from(input).unwrap();
        assert_eq!(*size_code, expected);
    }

    #[rstest]
    #[case::none(None, "-")]
    #[case::core30(Some(1), "TOPIX Core30")]
    #[case::large70(Some(2), "TOPIX Large70")]
    #[case::mid400(Some(4), "TOPIX Mid400")]
    #[case::small1(Some(6), "TOPIX Small 1")]
    #[case::small2(Some(7), "TOPIX Small 2")]
    fn test_display(#[case] input: Option<i16>, #[case] expected: &str) {
        let size_code = SizeCode::try_from(input).unwrap();
        assert_eq!(format!("{}", size_code), expected);
    }
}
