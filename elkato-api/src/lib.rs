pub mod model;

mod parser;
mod utils;

pub mod cors;
pub use model::*;

use crate::{
    cors::CorsProxy,
    utils::{date_filter_to_query, make_url},
};
use chrono::NaiveDate;
use futures::{stream, StreamExt, TryStreamExt};
use reqwest::header::{self, HeaderValue};
use std::sync::Arc;
use url::{ParseError, Url};

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct Credentials {
    pub username: String,
    pub password: String,
    pub club: String,
}

pub struct Api {
    client: reqwest::Client,
    frontend_url: Url,
    proxy: Arc<CorsProxy>,
    credentials: Credentials,
}

#[derive(Clone, Debug, Default)]
pub enum BookingState {
    #[default]
    Active,
    Inactive,
    All,
}

#[derive(Clone, Debug, Default)]
pub struct ListOptions {
    pub owner: Option<String>,
    pub start_from: Option<NaiveDate>,
    pub start_to: Option<NaiveDate>,
    pub end_from: Option<NaiveDate>,
    pub end_to: Option<NaiveDate>,
    pub state: BookingState,
}

impl Api {
    pub fn new(
        frontend_url: Url,
        proxy: CorsProxy,
        credentials: Credentials,
    ) -> anyhow::Result<Self> {
        let mut headers = header::HeaderMap::new();

        headers.insert(
            "Accept-Language",
            HeaderValue::from_static("de-DE;de;q=0.5"),
        );

        let client = reqwest::ClientBuilder::new()
            .default_headers(headers)
            .build()?;

        Ok(Self {
            client,
            frontend_url,
            proxy: Arc::new(proxy),
            credentials,
        })
    }

    fn url(&self, path: &str) -> Result<Url, ParseError> {
        self.frontend_url.join(path)
    }

    pub fn list_bookings(
        &self,
        options: ListOptions,
    ) -> impl TryStreamExt<Item = anyhow::Result<Booking>> {
        #[derive(Clone, Debug)]
        struct ListState {
            credentials: Credentials,
            offset: Option<usize>,
            client: reqwest::Client,
            url: Result<Url, ParseError>,
            proxy: Arc<CorsProxy>,
            frontend_url: Url,
            options: ListOptions,
        }

        let url = self.url("search.php");
        let frontend_url = self.frontend_url.clone();
        let client = self.client.clone();
        let proxy = self.proxy.clone();

        let init = ListState {
            offset: Some(0),
            url,
            proxy,
            frontend_url,
            client,
            credentials: self.credentials.clone(),
            options,
        };

        stream::try_unfold(init, move |state| {
            log::info!("Unfold: {state:?}");

            async move {
                let next = state.clone();

                match state.offset {
                    // having no offset means, we finish up in the last iteration
                    None => Result::<_, anyhow::Error>::Ok(None),
                    // having an offset means we need to pull in more data
                    Some(offset) => {
                        let builder = state
                            .client
                            .get(state.url.clone()?)
                            .basic_auth(
                                state.credentials.username.clone(),
                                Some(state.credentials.password.clone()),
                            )
                            .query(&[
                                ("club", state.credentials.club.clone()),
                                ("search_pos", format!("{}", offset)),
                                ("sel_room", "all".into()),
                                ("sel_booker", "all".into()),
                                (
                                    "sel_owner",
                                    state.options.owner.unwrap_or_else(|| "all".into()),
                                ),
                            ]);

                        let builder = builder.query(match &state.options.state {
                            BookingState::Active => &[("active", "on")][..],
                            BookingState::Inactive => &[("inactive", "on")][..],
                            BookingState::All => &[("active", "on"), ("inactive", "on")][..],
                        });

                        let builder = builder
                            .query(&date_filter_to_query("s_from", state.options.start_from));
                        let builder =
                            builder.query(&date_filter_to_query("s_to", state.options.start_to));
                        let builder =
                            builder.query(&date_filter_to_query("e_from", state.options.end_from));
                        let builder =
                            builder.query(&date_filter_to_query("e_to", state.options.end_to));

                        //let resp = builder.send().await?.error_for_status()?;
                        let resp = state
                            .proxy
                            .send(&state.client, builder)
                            .await?
                            .error_for_status()?;

                        log::debug!("URL: {}", resp.url());

                        let result = parser::parse_query(&resp.text().await?)?;

                        let next_offset = match result.paging {
                            None => None,
                            Some(p) if p.to >= p.total => None,
                            Some(p) => Some(p.to),
                        };

                        let context = (next.frontend_url.clone(), next.credentials.clone());

                        let y = stream::iter(result.bookings)
                            .map(move |b| {
                                let mut b = b.clone();
                                b.location = make_url(&b.id, &context.0, &context.1).ok();
                                b
                            })
                            .map(|b| Ok(b));

                        Ok(Some((
                            y,
                            ListState {
                                offset: next_offset,
                                ..next
                            },
                        )))
                    }
                }
            }
        })
        .try_flatten()
    }
}
