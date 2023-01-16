use crate::Booking;
use anyhow::anyhow;
use chrono::{DateTime, LocalResult, NaiveDateTime, Utc};
use itertools::Itertools;
use scraper::{ElementRef, Selector};

#[derive(Copy, Clone, Debug)]
pub struct Paging {
    pub from: usize,
    pub to: usize,
    pub total: usize,
}

#[derive(Clone, Debug)]
pub struct ListResponse {
    pub paging: Option<Paging>,
    pub bookings: Vec<Booking>,
}

pub(crate) fn parse_query(body: &str) -> anyhow::Result<ListResponse> {
    log::debug!("Payload: {}", body);

    if body.contains("<B>Die Suche ergab keine Treffer!</B>") {
        return Ok(ListResponse {
            paging: None,
            bookings: vec![],
        });
    }

    parse_bookings_list(body)
}

fn parse_bookings_list(body: &str) -> anyhow::Result<ListResponse> {
    let html = scraper::Html::parse_document(body);

    log::info!("Errors: {:?}", html.errors);
    log::info!("  Quirks Mode: {:?}", html.quirks_mode);
    //log::info!("  Tree: {:#?}", html.tree);

    let sel = selector(r##"#pageBody table[bordercolor="#000000"] tbody tr"##)?;
    //log::info!("Selector: {sel:?}");

    Ok(ListResponse {
        bookings: html
            .select(&sel)
            .filter_map::<anyhow::Result<Booking>, _>(|row| parse_row(row).transpose())
            .collect::<Result<Vec<_>, _>>()?,
        paging: None,
    })
}

const DATE_TIME_FORMAT: &str = "%d.%m.%y, %H:%M";

fn parse_row(row: ElementRef) -> anyhow::Result<Option<Booking>> {
    let cells: Vec<String> = row
        .select(&selector("td")?)
        .map(|cell| cell.text().collect())
        .collect();

    log::info!("Cells: {cells:?}");

    if let Some((id, resource, user, _, start, _, end, _duration, _, description)) =
        cells.into_iter().tuples().next()
    {
        let start = to_datetime(&start)?;
        let end = to_datetime(&end)?;

        Ok(Some(Booking {
            id,
            resource,
            user,
            start,
            end,
            description,
            location: None,
        }))
    } else {
        Ok(None)
    }
}

fn selector(sel: &str) -> anyhow::Result<Selector> {
    Ok(Selector::parse(sel).map_err(|err| anyhow!("Failed to parse selector: {err}"))?)
}

fn to_datetime(datetime: &str) -> anyhow::Result<DateTime<Utc>> {
    match NaiveDateTime::parse_from_str(datetime, DATE_TIME_FORMAT)?
        .and_local_timezone(chrono_tz::Europe::Berlin)
        .map(|dt| dt.with_timezone(&Utc))
    {
        LocalResult::None => Err(anyhow!("Failed to convert date/time")),
        LocalResult::Single(datetime) => Ok(datetime),
        LocalResult::Ambiguous(datetime, _) => Ok(datetime),
    }
}
