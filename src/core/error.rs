use crate::schema::errors::InputError;
use chrono::{naive::NaiveDate, offset::Utc};

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

pub fn departure_date_format(date: &str) -> Result<(), InputError> {
    let departure = NaiveDate::parse_from_str(date, "%Y-%m-%d");
    match departure {
        Err(_) => Err(InputError::DateFormatError),
        Ok(d) => {
            let today = Utc::today();
            if d.signed_duration_since(today.naive_utc()).num_days() > 0 {
                Ok(())
            } else {
                Err(InputError::InvalidDateError)
            }
        }
    }
}

#[cfg(test)]
mod date {
    use super::departure_date_format;
    use crate::schema::errors::InputError;

    #[test]
    fn date_is_correct() {
        assert!(departure_date_format("3020-01-20").is_ok());
    }

    #[test]
    fn date_should_be_yyyy_mm_dd() {
        assert_eq!(
            departure_date_format("2020/01/20").err().unwrap(),
            InputError::DateFormatError
        );
    }

    #[test]
    fn date_should_be_greater_than_today() {
        assert_eq!(
            departure_date_format("2019-01-20").err().unwrap(),
            InputError::InvalidDateError
        );
    }
}

#[cfg(test)]
mod iata {
    use super::iata_format;
    use crate::schema::errors::InputError;

    #[test]
    fn len_should_be_3() {
        assert_eq!(
            iata_format("IATA", "IAT").err().unwrap(),
            InputError::IataFormatError
        );

        assert_eq!(
            iata_format("IAT", "IATA").err().unwrap(),
            InputError::IataFormatError
        );
    }

    #[test]
    fn only_letters() {
        assert_eq!(
            iata_format("IAT", "I4T").err().unwrap(),
            InputError::IataFormatError
        );

        assert_eq!(
            iata_format("I&T", "IAT").err().unwrap(),
            InputError::IataFormatError
        );
    }
}
