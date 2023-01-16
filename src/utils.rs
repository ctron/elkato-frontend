use chrono::{DateTime, Duration, NaiveDate, TimeZone, Utc};

pub fn format_date<Tz>(date: &NaiveDate, now: &DateTime<Utc>, tz: &Tz) -> String
where
    Tz: TimeZone,
    Tz::Offset: std::fmt::Display,
{
    let now = now.with_timezone(tz).date_naive();
    let day = Duration::days(1);

    if &now == date {
        "Today".to_string()
    } else if &(now + day) == date {
        "Tomorrow".to_string()
    } else if &(now - day) == date {
        "Yesterday".to_string()
    } else {
        date.format("%v").to_string()
    }
}
