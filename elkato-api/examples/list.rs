use anyhow::anyhow;
use chrono::{Duration, Utc};
use elkato_api::{Api, Booking, Credentials, ListOptions};
use futures::stream::{StreamExt, TryStreamExt};
use url::Url;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();

    let api = Api::new(
        std::env::var("ELKATO_URL")
            .map_err(|err| anyhow!(err))
            .and_then(|url| Ok(Url::parse(&url)?))
            .or_else(|_| Ok::<_, anyhow::Error>(Url::parse("https://www.elkato.de/buchung/")?))?,
        Credentials {
            username: std::env::var("ELKATO_USERNAME")?,
            password: std::env::var("ELKATO_PASSWORD")?,
            club: std::env::var("ELKATO_CLUB")?,
        },
    )?;

    let today = Utc::now().date_naive();

    let bookings: Vec<Booking> = api
        .list_bookings(ListOptions {
            owner: Some("demo".into()),
            start_from: Some(today - Duration::days(7)),
            end_to: Some(today + Duration::days(7)),
            ..Default::default()
        })
        .boxed_local()
        .try_collect()
        .await?;

    println!("Bookings ({}):", bookings.len());

    for booking in bookings {
        println!("{booking:?}");
    }

    Ok(())
}
