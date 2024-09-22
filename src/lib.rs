use std::{collections::BTreeMap, sync::Arc};

pub mod types;
pub use types::*;

const BASE_URL: &str = "https://tetter-server.vercel.app";

// Helper function to call the API server with a method, path, and optional data.
pub(crate) async fn call_api(
    client: &Client,
    method: reqwest::Method,
    path: &str,
    data: Option<serde_json::Value>,
    form: Option<BTreeMap<String, String>>,
) -> ApiResult<reqwest::Response> {
    let url = format!("{BASE_URL}/{path}");
    let mut query = client.client.request(method, &url);

    if let Some(data) = data {
        query = query.json(&data);
    }

    if let Some(form) = form {
        query = query.form(&form);
    }

    let res = query.send().await?;

    if res.status().is_success() {
        Ok(res)
    } else {
        let tetter_error = res.json::<TetterError>().await?;
        Err(Error::Tetter(tetter_error))
    }
}

pub struct Client {
    client: Arc<reqwest::Client>,
}

impl Client {
    /// Creates a new client with an optional token.
    ///
    /// A token is required for write operations.
    pub fn new(token: Option<String>) -> Self {
        let mut headers = reqwest::header::HeaderMap::new();

        // sanitize token to idiot-proof the crate a bit
        let token = token
            .map(|t| t.trim().to_string())
            .filter(|t| !t.is_empty())
            .map(|t| t.trim_start_matches("Bearer ").to_string());

        if let Some(token) = token {
            headers.insert(
                reqwest::header::AUTHORIZATION,
                reqwest::header::HeaderValue::from_str(&format!("Bearer {}", token)).unwrap(),
            );
        }

        let client = reqwest::Client::builder()
            .use_rustls_tls()
            .user_agent("Kasane/0.1")
            .default_headers(headers)
            .build()
            .unwrap();
        Self {
            client: Arc::new(client),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;

    #[test]
    fn parse_example_post() {
        let post_str = include_str!("../test/tet.json");

        tracing::info!("Parsing post: {}", post_str);

        // Try to parse the tet
        let tet: Tet = serde_json::from_str(post_str).unwrap();

        println!("{:#?}", tet);
    }
    #[test]
    fn parse_example_thread() {
        let thread_str = include_str!("../test/thread.json");

        tracing::info!("Parsing thread: {}", thread_str);

        // Try to parse the thread
        let thread: Vec<Thread> = serde_json::from_str(thread_str).unwrap();

        println!("{:#?}", thread);
    }

    #[ignore = "Requires authentication"]
    #[test(tokio::test)]
    async fn get_current_user() {
        let client = Client::new(None);
        let u = User::get_current_user(&client).await.unwrap();
        println!("{:#?}", u);
    }
}
