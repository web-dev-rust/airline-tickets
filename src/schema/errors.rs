use juniper::{FieldError, IntoFieldError};
use redis;

#[derive(Debug, Clone, PartialEq)]
pub enum GenericError {
    InputError(InputError),
    InternalError(InternalError),
    RedisError(RedisError)
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

#[derive(Debug, Clone, PartialEq)]
pub enum RedisError {
    ClientError,
    ConnectionError,
    TransactionError,
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
            GenericError::RedisError(RedisError::ConnectionError) => FieldError::new(
                "Connection error with redis",
                graphql_value!({
                    "type": "REDIS CONNECTION ERROR"
                }),
            ),
            GenericError::RedisError(RedisError::TransactionError) => FieldError::new(
                "Transaction with redis failed",
                graphql_value!({
                    "type": "REDIS TRANSACTION ERROR"
                }),
            ),
            GenericError::RedisError(RedisError::ClientError) => FieldError::new(
                "Client failed to communicate with redis",
                graphql_value!({
                    "type": "REDIS CLIENT ERROR"
                }),
            ),
        }
    }
}


impl From<reqwest::Error> for GenericError {
    fn from(_: reqwest::Error) -> Self {
        GenericError::InternalError(InternalError::RequestFailedError)
    }
}

impl From<serde_json::Error> for GenericError {
    fn from(_: serde_json::Error) -> Self {
        GenericError::InternalError(InternalError::ResponseParseError)
    }
}

impl From<redis::RedisError> for GenericError {
    fn from(e: redis::RedisError) -> Self {
        match e {
            e1 if redis::RedisError::is_timeout(&e1) => 
                GenericError::RedisError(RedisError::ConnectionError),
            e2 if redis::RedisError::is_io_error(&e2) =>
                GenericError::RedisError(RedisError::TransactionError),
            e3 if redis::RedisError::is_connection_refusal(&e3) => 
                GenericError::RedisError(RedisError::ClientError),
            _ => GenericError::RedisError(RedisError::ConnectionError),
        }
    }
}