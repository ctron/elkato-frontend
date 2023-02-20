use elkato_api::Credentials;
use yew::prelude::*;
use yew_hooks::{use_local_storage, UseLocalStorageHandle};

const KEY_CREDENTIALS: &str = "credentials";

#[derive(Clone, PartialEq)]
pub struct Session {
    pub credentials: UseLocalStorageHandle<Credentials>,
}

impl Session {
    pub fn login(&self, credentials: Credentials) {
        self.credentials.set(credentials);
    }

    pub fn logout(&self) {
        self.credentials.delete();
    }
}

#[hook]
pub fn use_session() -> UseLocalStorageHandle<Credentials> {
    use_local_storage::<Credentials>(KEY_CREDENTIALS.into())
}
