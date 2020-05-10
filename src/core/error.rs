use crate::schema::errors::InputError;

pub fn iata_format(origin: &str, destination: &str) -> Result<(), InputError> {
    if origin.len() != 3
        || !origin.chars().all(char::is_alphabetic)
        || destination.len() != 3
        || !destination.chars().all(char::is_alphabetic)
    {
        Err(InputError::IataFormatError)
    } else {
        Ok(())
    }
}
