use crate::errors::TodoError;
use chrono::NaiveDate;

pub fn parse_date(date: &str) -> Result<NaiveDate, TodoError> {
    NaiveDate::parse_from_str(date, "%d-%m-%Y")
        .map_err(|_| TodoError::DateInvalide(date.to_string()))
}
