use juniper::{FieldError, IntoFieldError};

// fix naming
#[derive(Debug, Clone, PartialEq)]
pub enum InputError {
    IataFormatError,
    DateFormatError,
    InvalidDateError,
}

impl IntoFieldError for InputError {
    fn into_field_error(self) -> FieldError {
        match self {
            InputError::IataFormatError => FieldError::new(
                "The IATA format for origin and destinantion consists of 3 letter",
                graphql_value!({
                    "type": "IATA FORMAT ERROR"
                }),
            ),
            InputError::DateFormatError => FieldError::new(
                "departure date should be formated yyyy-mm-dd",
                graphql_value!({
                    "type": "DATE FORMAT ERROR"
                }),
            ),
            InputError::InvalidDateError => FieldError::new(
                "Date should be greater than today",
                graphql_value!({
                    "type": "INVALID DATE ERROR"
                }),
            ),
        }
    }
}
