/// Response module for standardized API responses
/// Ensures all responses follow a consistent structure

use std::fmt;

/// Standard response structure for all API responses
#[derive(Debug, Clone)]
pub struct ApiResponse<T> {
    pub status: ResponseStatus,
    pub data: Option<T>,
    pub error: Option<String>,
    pub message: String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ResponseStatus {
    Success,
    Error,
}

impl fmt::Display for ResponseStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ResponseStatus::Success => write!(f, "success"),
            ResponseStatus::Error => write!(f, "error"),
        }
    }
}

impl<T: fmt::Display> ApiResponse<T> {
    /// Create a successful response
    pub fn success(data: T, message: &str) -> Self {
        ApiResponse {
            status: ResponseStatus::Success,
            data: Some(data),
            error: None,
            message: message.to_string(),
        }
    }

    /// Create an error response
    pub fn error(error: &str, message: &str) -> ApiResponse<T> {
        ApiResponse {
            status: ResponseStatus::Error,
            data: None,
            error: Some(error.to_string()),
            message: message.to_string(),
        }
    }

    /// Display the response in a user-friendly format
    pub fn display(&self) -> String {
        match self.status {
            ResponseStatus::Success => {
                format!("✅ {}\nData: {}", self.message, self.data.as_ref().unwrap())
            }
            ResponseStatus::Error => {
                format!(
                    "❌ {}\nError: {}",
                    self.message,
                    self.error.as_ref().unwrap_or(&"Unknown error".to_string())
                )
            }
        }
    }
}

/// Response for operations without data
#[derive(Debug, Clone)]
pub struct SimpleResponse {
    pub status: ResponseStatus,
    pub message: String,
    pub error: Option<String>,
}

impl SimpleResponse {
    /// Create a successful response
    pub fn success(message: &str) -> Self {
        SimpleResponse {
            status: ResponseStatus::Success,
            message: message.to_string(),
            error: None,
        }
    }

    /// Create an error response
    pub fn error(message: &str) -> Self {
        SimpleResponse {
            status: ResponseStatus::Error,
            message: message.to_string(),
            error: Some(message.to_string()),
        }
    }

    /// Display the response in a user-friendly format
    pub fn display(&self) -> String {
        match self.status {
            ResponseStatus::Success => format!("✅ {}", self.message),
            ResponseStatus::Error => format!("❌ {}", self.message),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success_response() {
        let response = ApiResponse::success("test_data".to_string(), "Operation successful");
        assert_eq!(response.status, ResponseStatus::Success);
        assert!(response.data.is_some());
        assert!(response.error.is_none());
    }

    #[test]
    fn test_error_response() {
        let response: ApiResponse<String> =
            ApiResponse::error("validation_error", "Invalid input provided");
        assert_eq!(response.status, ResponseStatus::Error);
        assert!(response.data.is_none());
        assert!(response.error.is_some());
    }

    #[test]
    fn test_simple_response_success() {
        let response = SimpleResponse::success("Operation completed");
        assert_eq!(response.status, ResponseStatus::Success);
        assert!(response.error.is_none());
    }

    #[test]
    fn test_simple_response_error() {
        let response = SimpleResponse::error("Something went wrong");
        assert_eq!(response.status, ResponseStatus::Error);
        assert!(response.error.is_some());
    }
}

/// Health status data for the health check endpoint
#[derive(Debug, Clone)]
pub struct HealthStatus {
    pub status: String,
    pub version: String,
    pub service: String,
}

impl fmt::Display for HealthStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "service: {}, version: {}, status: {}",
            self.service, self.version, self.status
        )
    }
}

#[cfg(test)]
mod health_tests {
    use super::*;

    #[test]
    fn test_health_status_display() {
        let health = HealthStatus {
            status: "ok".to_string(),
            version: "0.1.0".to_string(),
            service: "stellarforge-cli".to_string(),
        };
        let display = format!("{}", health);
        assert!(display.contains("ok"));
        assert!(display.contains("stellarforge-cli"));
    }

    #[test]
    fn test_health_check_response() {
        let health = HealthStatus {
            status: "ok".to_string(),
            version: "0.1.0".to_string(),
            service: "stellarforge-cli".to_string(),
        };
        let response = ApiResponse::success(health, "Health check passed");
        assert_eq!(response.status, ResponseStatus::Success);
        assert!(response.data.is_some());
        assert!(response.error.is_none());
        assert_eq!(response.message, "Health check passed");
    }
}
