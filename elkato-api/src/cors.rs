use reqwest::RequestBuilder;
use url::Url;

#[derive(Clone, Debug)]
pub enum CorsProxy {
    None,
    Prepend(Url),
    Query { url: Url, parameter: String },
}

impl CorsProxy {
    pub async fn send(
        &self,
        client: &reqwest::Client,
        builder: RequestBuilder,
    ) -> Result<reqwest::Response, reqwest::Error> {
        let mut req = builder.build()?;

        match self {
            Self::None => {}
            Self::Prepend(proxy) => {
                let mut proxy = proxy.clone();
                proxy.set_path(req.url().as_str());
                *req.url_mut() = proxy;
            }
            Self::Query { url, parameter } => {
                let mut proxy = url.clone();
                proxy
                    .query_pairs_mut()
                    .append_pair(parameter, req.url().as_str());
                *req.url_mut() = proxy;
            }
        }

        client.execute(req).await
    }
}
