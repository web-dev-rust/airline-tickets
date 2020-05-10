use juniper::{FieldError, IntoFieldError};

pub enum InputError {
    IataFormatError,
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
        }
    }
}
