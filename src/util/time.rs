use chrono::NaiveDate;
use util::errors::AppResult;

pub fn date_from_str(date_str: &str) -> AppResult<NaiveDate> {
    NaiveDate::parse_from_str(date_str, "%Y-%m-%d").map_err(|e| app_error!(TimeParseError, e))
}
