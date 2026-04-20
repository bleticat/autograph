use crate::shared::error::AppErr;
use chrono::{DateTime, NaiveDate, Utc};

pub fn parse_date(s: &str) -> Result<DateTime<Utc>, AppErr> {
    let s = s.trim();
    if s.is_empty() {
        return Err(AppErr::Parse("date is required".to_owned()));
    }
    let date = NaiveDate::parse_from_str(s, "%Y-%m-%d")
        .map_err(|e| AppErr::Parse(format!("invalid date, expected YYYY-MM-DD: {e}")))?;
    Ok(date.and_hms_opt(0, 0, 0).expect("valid midnight").and_utc())
}

pub fn parse_optional_date(s: Option<&str>) -> Result<Option<DateTime<Utc>>, AppErr> {
    let Some(s) = s.map(str::trim).filter(|s| !s.is_empty()) else {
        return Ok(None);
    };
    let date = NaiveDate::parse_from_str(s, "%Y-%m-%d")
        .map_err(|e| AppErr::Parse(format!("invalid date, expected YYYY-MM-DD: {e}")))?;
    Ok(Some(
        date.and_hms_opt(0, 0, 0).expect("valid midnight").and_utc(),
    ))
}

#[cfg(test)]
mod tests {
    use super::{parse_date, parse_optional_date};
    use chrono::NaiveDate;

    #[test]
    fn parse_date_accepts_valid_iso_date() {
        assert_eq!(
            parse_date("2026-05-10").unwrap().date_naive(),
            NaiveDate::from_ymd_opt(2026, 5, 10).unwrap()
        );
    }

    #[test]
    fn parse_date_rejects_empty_string() {
        assert!(parse_date("   ").is_err());
    }

    #[test]
    fn parse_optional_date_accepts_valid_iso_date() {
        assert_eq!(
            parse_optional_date(Some("2026-05-10"))
                .unwrap()
                .map(|d| d.date_naive()),
            Some(NaiveDate::from_ymd_opt(2026, 5, 10).unwrap())
        );
    }

    #[test]
    fn parse_optional_date_normalizes_empty_to_none() {
        assert_eq!(parse_optional_date(Some("   ")).unwrap(), None);
    }

    #[test]
    fn parse_optional_date_rejects_invalid_date() {
        assert!(parse_optional_date(Some("2026-13-10")).is_err());
    }
}
