use chrono::DateTime;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Booking {
    pub id: String,
    pub resource: String,
    pub user: String,
    pub start: DateTime<Utc>,
    pub end: DateTime<Utc>,
    pub description: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<Url>,
}

impl Booking {
    pub fn is_active(&self, now: &DateTime<Utc>) -> bool {
        now >= &self.start && now <= &self.end
    }
}
