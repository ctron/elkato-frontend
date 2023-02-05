use crate::utils::format_date;
use chrono::{DateTime, Duration, Utc};
use chrono_tz::Europe::Berlin;
use elkato_api::*;
use futures::{StreamExt, TryStreamExt};
use patternfly_yew::*;
use url::Url;
use yew::prelude::*;
use yew_hooks::{use_async_with_options, UseAsyncOptions};

async fn bookings(credentials: Credentials) -> anyhow::Result<Vec<Booking>> {
    log::info!("Load bookings");

    let owner = credentials.username.clone();
    let api = Api::new(
        Url::parse(crate::app::FRONTEND_URL)?,
        crate::app::cors_proxy(),
        credentials,
    )?;

    let today = Utc::now().date_naive();

    log::info!("Load bookings (begin)");

    api.list_bookings(ListOptions {
        owner: Some(owner),
        start_from: Some(today - Duration::days(7)),
        end_to: Some(today + Duration::days(7)),
        ..Default::default()
    })
    .boxed_local()
    .try_collect()
    .await
}

fn select(mut bookings: Vec<Booking>) -> Vec<Booking> {
    // sort by time asc
    bookings.sort_by(|a, b| a.start.cmp(&b.start).then_with(|| a.end.cmp(&b.end)));

    log::info!("Bookings: {:?}", bookings);

    let now = Utc::now();

    let mut new = Vec::new();

    for b in bookings.iter().rev() {
        if b.end >= now {
            new.push(b.clone());
        } else {
            new.push(b.clone());
            break;
        }
    }

    new.reverse();
    new
}

#[derive(Properties, Clone, Debug, PartialEq, Eq)]
pub struct Props {
    pub credentials: Credentials,
}

#[function_component(Index)]
pub fn index(props: &Props) -> Html {
    let credentials = props.credentials.clone();
    let bookings = use_async_with_options(
        async move {
            {
                let bookings = bookings(credentials).await;

                log::info!("Load bookings (done): {bookings:?}");

                bookings.map_err(|err| err.to_string()).map(select)
            }
        },
        UseAsyncOptions::enable_auto(),
    );

    html!(<>
        <PageSection variant={PageSectionVariant::Light}>
            <Title level={Level::H1} size={Size::XXXXLarge}>{ "Bookings" }</Title>
        </PageSection>
        <PageSection variant={PageSectionVariant::Light}> {
            match (bookings.loading, &bookings.data, &bookings.error) {
                (true, _, _) => html!({ "Loading, ..." }),
                (false, Some(bookings), _) => html!(
                    <>
                        <Bookings bookings={bookings.clone()}/>
                    </>
                ),
                (false, _, Some(error)) => html!(
                    <>
                        {format!("Error (X): {error}")}
                    </>
                ),
                _ => html!(),
            }
        } </PageSection>
    </>)
}

#[derive(Properties, PartialEq, Eq)]
pub struct BookingProps {
    pub bookings: Vec<Booking>,
}

#[function_component(Bookings)]
fn bookings(props: &BookingProps) -> Html {
    html!(
        <PageSection>
            <Gallery gutter=true>
                { for props.bookings.iter().cloned().map(|booking| {
                    html!(
                        <BookingCard {booking} />
                    )
                })}
            </Gallery>
        </PageSection>
    )
}

#[derive(Properties, PartialEq, Eq)]
pub struct BookingCardProps {
    pub booking: Booking,
}

#[function_component(BookingCard)]
fn booking_card(props: &BookingCardProps) -> Html {
    let now = Utc::now();
    html!(
        <Card
            selectable=true
            onclick={make_onclick(&props.booking)}
            selected={props.booking.is_active(&now)}
            title={html!{<>
                { title(&props.booking, &now) }
            </>}}
            >
            <div>{ &props.booking.resource }</div>
            if !props.booking.description.is_empty() {
                <div>{ &props.booking.description }</div>
            }

        </Card>
    )
}

fn make_onclick<E>(sel_booking: &Booking) -> Callback<E> {
    let loc = sel_booking.location.clone();
    Callback::from(move |_| {
        if let Some(url) = &loc {
            log::info!("Opening: {url}");
            gloo_utils::window()
                .open_with_url_and_target(url.as_str(), "_blank")
                .ok();
        }
    })
}

fn title(booking: &Booking, now: &DateTime<Utc>) -> String {
    let dur = booking.end - booking.start;

    let dur = format_duration(&dur);

    let tz = &Berlin;

    let start_date = booking.start.with_timezone(tz).date_naive();
    let end_date = booking.end.with_timezone(tz).date_naive();

    if start_date == end_date {
        let date = format_date(&start_date, now, tz);
        format!(
            "{} | {} → {} ({})",
            date,
            booking.start.with_timezone(tz).format("%H:%M"),
            booking.end.with_timezone(tz).format("%H:%M"),
            dur
        )
    } else {
        let start_date = format_date(&start_date, now, tz);
        let end_date = format_date(&end_date, now, tz);
        format!(
            "{} {} → {} {} ({})",
            start_date,
            booking.start.with_timezone(tz).format("%H:%M"),
            end_date,
            booking.end.with_timezone(tz).format("%H:%M"),
            dur
        )
    }
}

fn format_duration(duration: &Duration) -> String {
    let mins = duration.num_minutes();
    if mins < 60 {
        format!("{} min", mins)
    } else {
        let hours = duration.num_hours();
        let rem = *duration - Duration::hours(hours);
        match rem.num_minutes() {
            0 => format!("{} h", hours),
            15 => format!("{} ¼ h", hours),
            30 => format!("{} ½ h", hours),
            45 => format!("{} ¾ h", hours),
            mins => format!("{} h {} min", hours, mins),
        }
    }
}
