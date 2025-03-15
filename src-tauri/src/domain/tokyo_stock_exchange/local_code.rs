use crate::domain::DomainError;

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct LocalCode(String);

impl core::ops::Deref for LocalCode {
    type Target = String;

    fn deref(&'_ self) -> &'_ Self::Target {
        &self.0
    }
}

impl TryFrom<String> for LocalCode {
    type Error = anyhow::Error;

    fn try_from(value: String) -> Result<Self, anyhow::Error> {
        let local_code = Self(value);

        local_code.validate()?;

        Ok(local_code)
    }
}

impl LocalCode {
    pub fn validate(&self) -> anyhow::Result<()> {
        let value = self.as_str();

        if value.is_empty() {
            // When the local code is empty
            Err(DomainError::ValidationError("Local code is empty".to_string()).into())
        } else if value.len() != 4 {
            // When the local code is not 4 digits
            Err(DomainError::ValidationError("Local code must be 4 digits".to_string()).into())
        } else if !value
            .chars()
            .all(|c| c.is_ascii_digit() || c.is_ascii_alphabetic())
        {
            // When there are characters other than numbers and alphabets
            Err(DomainError::ValidationError("Local code must be alphanumeric".to_string()).into())
        } else {
            // When the local code is valid
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use rstest::*;

    #[rstest]
    #[case::success("1234")]
    #[case::success_with_alphabet("123A")]
    fn test_validate_local_code_success(#[case] local_code: &str) {
        let local_code = LocalCode::try_from(local_code.to_string());

        assert!(local_code.is_ok());
    }

    #[rstest]
    #[case::empty("", "Local code is empty")]
    #[case::not_4_digits("123", "Local code must be 4 digits")]
    #[case::not_4_digits("12345", "Local code must be 4 digits")]
    #[case::not_alphanumeric("123$", "Local code must be alphanumeric")]
    #[case::not_alphanumeric("1 23", "Local code must be alphanumeric")]
    fn test_validate_local_code_error(#[case] local_code: &str, #[case] expected_error: &str) {
        let local_code = LocalCode::try_from(local_code.to_string());

        assert!(local_code.is_err());

        let error: DomainError = local_code.unwrap_err().try_into().unwrap();
        let error_message: String = error.into();
        assert_eq!(error_message, expected_error);
    }
}
