use crate::pages;
use elkato_api::{CorsProxy, Credentials};
use pages::{index::Index, Pages};
use patternfly_yew::*;
use url::Url;
use yew::prelude::*;
use yew_hooks::{use_session_storage, UseSessionStorageHandle};
use yew_nested_router::{prelude::*, Switch as RouterSwitch};

pub const FRONTEND_URL: &str = "https://www.elkato.de/buchung/";

#[cfg(not(debug_assertions))]
pub fn cors_proxy() -> CorsProxy {
    pub const CORS_API_URL: &str = "https://elkato.dentrassi.de/proxy.php";

    CorsProxy::Query {
        url: Url::parse(CORS_API_URL).unwrap(),
        parameter: "url".to_string(),
    }
}

#[cfg(debug_assertions)]
pub fn cors_proxy() -> CorsProxy {
    pub const CORS_API_URL: &str = "http://localhost:9999";

    CorsProxy::Prepend(Url::parse(CORS_API_URL).unwrap())
}

const KEY_CREDENTIALS: &str = "credentials";

#[derive(Clone, PartialEq)]
pub struct Session {
    pub credentials: UseSessionStorageHandle<Credentials>,
}

impl Session {
    pub fn login(&self, credentials: Credentials) {
        self.credentials.set(credentials);
    }

    pub fn logout(&self) {
        self.credentials.delete();
    }
}

#[function_component(Application)]
pub fn app() -> Html {
    let credentials = use_session_storage::<Credentials>(KEY_CREDENTIALS.into());

    log::info!("Credentials: {:?}", *credentials);

    let session = Session { credentials };

    html!(
        <>
            <ContextProvider<Session> context={session.clone()}>
                {
                    match &*session.credentials {
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
            </ContextProvider<Session>>
        </>
    )
}

#[derive(Clone, Debug, Properties, PartialEq, Eq)]
pub struct WithCredentialsProps {
    pub credentials: Credentials,
}

#[function_component(WithCredentials)]
pub fn with_credentials(props: &WithCredentialsProps) -> Html {
    let credentials = props.credentials.clone();
    let session = use_context::<Session>().unwrap();

    let logout = {
        Callback::from(move |_| {
            session.logout();
        })
    };

    let tools = html!(
        <>
            <Button icon={Icon::PowerOffIcon} onclick={logout} />
        </>
    );

    html!(
        <Page {tools}>
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
