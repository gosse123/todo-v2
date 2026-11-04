use crate::errors::TodoError;
use chrono::NaiveDate;

pub fn parse_date(date: &str) -> Result<NaiveDate, TodoError> {
    NaiveDate::parse_from_str(date, "%d-%m-%Y")
        .map_err(|_| TodoError::DateInvalide(date.to_string()))
}

#[cfg(test)]
mod tests {
    use super::parse_date;
    use crate::errors::TodoError;

    #[test]
    fn parse_une_date_au_format_jour_mois_annee() {
        let date = parse_date("01-02-2026").unwrap();

        assert_eq!(date, chrono::NaiveDate::from_ymd_opt(2026, 2, 1).unwrap());
    }

    #[test]
    fn refuse_une_date_au_format_iso() {
        assert!(matches!(
            parse_date("2026-02-01"),
            Err(TodoError::DateInvalide(_))
        ));
    }

    #[test]
    fn refuse_une_date_vide() {
        assert!(matches!(parse_date(""), Err(TodoError::DateInvalide(_))));
    }
}
