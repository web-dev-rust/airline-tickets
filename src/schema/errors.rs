use juniper::{FieldError, IntoFieldError};

#[derive(Debug, Clone, PartialEq)]
pub enum GenericError {
    InputError(InputError),
    InternalError(InternalError),
}

#[derive(Debug, Clone, PartialEq)]
pub enum InputError {
    IataFormatError,
    DateFormatError,
    InvalidDateError,
}

#[derive(Debug, Clone, PartialEq)]
pub enum InternalError {
    RequestFailedError,
    ResponseParseError,
}

impl IntoFieldError for GenericError {
    fn into_field_error(self) -> FieldError {
        match self {
            GenericError::InputError(InputError::IataFormatError) => FieldError::new(
                "The IATA format for origin and destinantion consists of 3 letter",
                graphql_value!({
                    "type": "IATA FORMAT ERROR"
                }),
            ),
            GenericError::InputError(InputError::DateFormatError) => FieldError::new(
                "departure date should be formated yyyy-mm-dd",
                graphql_value!({
                    "type": "DATE FORMAT ERROR"
                }),
            ),
            GenericError::InputError(InputError::InvalidDateError) => FieldError::new(
                "Date should be greater than today",
                graphql_value!({
                    "type": "INVALID DATE ERROR"
                }),
            ),
            GenericError::InternalError(InternalError::RequestFailedError) => FieldError::new(
                "Could not complete properly request to the backend",
                graphql_value!({
                    "type": "REQUEST FAILED ERROR"
                }),
            ),
            GenericError::InternalError(InternalError::ResponseParseError) => FieldError::new(
                "Could not parse response from backend",
                graphql_value!({
                    "type": "RESPONSE PARSE ERROR"
                }),
            ),
        }
    }
}


impl From<reqwest::Error> for GenericError {
    fn from(e: reqwest::Error) -> Self {
        GenericError::InternalError(InternalError::RequestFailedError)
    }
}

impl From<serde_json::Error> for GenericError {
    fn from(e: serde_json::Error) -> Self {
        GenericError::InternalError(InternalError::ResponseParseError)
    }
}