use crate::pages;
use elkato_api::Credentials;
use pages::{index::Index, Pages};
use patternfly_yew::*;
use yew::prelude::*;
use yew_hooks::use_session_storage;
use yew_nested_router::{prelude::*, Switch as RouterSwitch};

pub const API_URL: &str = "https://www.elkato.de/buchung/";
pub const CORS_API_URL: &str = "http://localhost:9999/https://www.elkato.de/buchung/";

#[function_component(Application)]
pub fn app() -> Html {
    let credentials = use_session_storage::<Credentials>("credentials".into());

    log::info!("Credentials: {:?}", *credentials);

    match &*credentials {
        Some(credentials) => {
            let credentials = credentials.clone();
            html!(
                <WithCredentials {credentials} />
            )
        }
        None => html!(
            <pages::login::Login/>
        ),
    }
}

#[derive(Clone, Debug, Properties, PartialEq, Eq)]
pub struct WithCredentialsProps {
    pub credentials: Credentials,
}

#[function_component(WithCredentials)]
pub fn with_credentials(props: &WithCredentialsProps) -> Html {
    let credentials = props.credentials.clone();
    html!(
        <Page>
            <Router<Pages> default={Pages::Index}>
                <RouterSwitch<Pages> render={move |target| match target {
                    Pages::Index => html!(<Index credentials={credentials.clone()}/>)
                }
            }/>
            </Router<Pages>>
        </Page>
    )
}

#[function_component(Main)]
pub fn main() -> Html {
    html!(
        <ToastViewer>
            <BackdropViewer>
                <Application/>
            </BackdropViewer>
        </ToastViewer>
    )
}
