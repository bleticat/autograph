use crate::shared::error::AppErr;
use time::{Date, OffsetDateTime, Time, format_description::well_known::Iso8601};

pub fn parse_date(s: &str) -> Result<OffsetDateTime, AppErr> {
    let s = s.trim();
    if s.is_empty() {
        return Err(AppErr::Parse("date is required".to_owned()));
    }
    let date = Date::parse(s, &Iso8601::DEFAULT)
        .map_err(|e| AppErr::Parse(format!("invalid date, expected YYYY-MM-DD: {e}")))?;
    Ok(date.with_time(Time::MIDNIGHT).assume_utc())
}

pub fn parse_optional_date(s: Option<&str>) -> Result<Option<OffsetDateTime>, AppErr> {
    let Some(s) = s.map(str::trim).filter(|s| !s.is_empty()) else {
        return Ok(None);
    };
    let date = Date::parse(s, &Iso8601::DEFAULT)
        .map_err(|e| AppErr::Parse(format!("invalid date, expected YYYY-MM-DD: {e}")))?;
    Ok(Some(date.with_time(Time::MIDNIGHT).assume_utc()))
}

#[cfg(test)]
mod tests {
    use super::{parse_date, parse_optional_date};
    use time::{Date, Month};

    #[test]
    fn parse_date_accepts_valid_iso_date() {
        assert_eq!(
            parse_date("2026-05-10").unwrap().date(),
            Date::from_calendar_date(2026, Month::May, 10).unwrap()
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
                .map(|d| d.date()),
            Some(Date::from_calendar_date(2026, Month::May, 10).unwrap())
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
