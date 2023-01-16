use crate::Credentials;
use chrono::{Datelike, NaiveDate};
use url::{ParseError, Url};

pub(crate) fn date_filter_to_query(prefix: &str, date: Option<NaiveDate>) -> Vec<(String, String)> {
    match date {
        Some(d) => vec![
            (prefix.to_string(), "1".into()),
            (format!("{}_day", prefix), d.day().to_string()),
            (format!("{}_month", prefix), d.month().to_string()),
            (format!("{}_year", prefix), d.year().to_string()),
        ],
        None => vec![(prefix.to_string(), "0".into())],
    }
}

/// Create the URL for a booking
pub(crate) fn make_url(id: &str, url: &Url, credentials: &Credentials) -> Result<Url, ParseError> {
    let mut url = url.join(&format!("/buchung/view_entry.php"))?;

    url.query_pairs_mut()
        .clear()
        .append_pair("club", &credentials.club)
        .append_pair("id", id);

    Ok(url)
}
