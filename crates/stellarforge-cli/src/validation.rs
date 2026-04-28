/// Validation module for CLI input validation
/// Provides validation functions for required fields and friendly error messages

pub struct ValidationError {
    pub field: String,
    pub message: String,
}

impl ValidationError {
    pub fn new(field: &str, message: &str) -> Self {
        ValidationError {
            field: field.to_string(),
            message: message.to_string(),
        }
    }

    pub fn display(&self) -> String {
        format!("❌ {}: {}", self.field, self.message)
    }
}

pub struct Validator;

impl Validator {
    /// Validate that a required field is not empty
    pub fn required(value: &Option<String>, field_name: &str) -> Result<String, ValidationError> {
        match value {
            Some(v) if !v.trim().is_empty() => Ok(v.trim().to_string()),
            _ => Err(ValidationError::new(field_name, "This field is required")),
        }
    }

    /// Validate that a string is not empty
    pub fn required_string(value: &str, field_name: &str) -> Result<String, ValidationError> {
        if value.trim().is_empty() {
            Err(ValidationError::new(field_name, "This field is required"))
        } else {
            Ok(value.trim().to_string())
        }
    }

    /// Validate that a value is a valid network
    pub fn valid_network(network: &str) -> Result<String, ValidationError> {
        match network {
            "futurenet" | "testnet" | "mainnet" => Ok(network.to_string()),
            _ => Err(ValidationError::new(
                "network",
                "Must be one of: futurenet, testnet, mainnet",
            )),
        }
    }

    /// Validate that a contract name is valid
    pub fn valid_contract(name: &str) -> Result<String, ValidationError> {
        match name {
            "vesting" | "stream" | "multisig" | "governor" | "oracle" => Ok(name.to_string()),
            _ => Err(ValidationError::new(
                "contract",
                "Must be one of: vesting, stream, multisig, governor, oracle",
            )),
        }
    }

    /// Validate multiple errors and return all at once
    pub fn validate_all(errors: Vec<ValidationError>) -> Result<(), Vec<ValidationError>> {
        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_required_field_empty() {
        let result = Validator::required(&None, "name");
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().message, "This field is required");
    }

    #[test]
    fn test_required_field_valid() {
        let result = Validator::required(&Some("test".to_string()), "name");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "test");
    }

    #[test]
    fn test_valid_network() {
        assert!(Validator::valid_network("testnet").is_ok());
        assert!(Validator::valid_network("mainnet").is_ok());
        assert!(Validator::valid_network("futurenet").is_ok());
        assert!(Validator::valid_network("invalid").is_err());
    }

    #[test]
    fn test_valid_contract() {
        assert!(Validator::valid_contract("vesting").is_ok());
        assert!(Validator::valid_contract("stream").is_ok());
        assert!(Validator::valid_contract("invalid").is_err());
    }
}
