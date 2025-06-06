
use reqwest::{
    header::{HeaderMap, HeaderValue, ACCEPT, CONTENT_TYPE},
    Client,
};
use serde::{de::DeserializeOwned, Serialize};

#[allow(dead_code)]
pub enum RequestType {
    GET,
    PUT,
}

#[allow(dead_code, unused_variables)]
pub struct HttpClient {
    client: Client,
    get_headers: HeaderMap,
    post_headers: HeaderMap,
}

impl Default for HttpClient {
    fn default() -> Self {
        let mut get_headers = HeaderMap::new();
        get_headers.insert(
            ACCEPT,
            HeaderValue::from_static("application/json, text/plain, */*"),
        );

        let mut post_headers = HeaderMap::new();
        post_headers.insert(
            ACCEPT,
            HeaderValue::from_static("application/json, text/plain, */*"),
        );
        post_headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

        Self {
            client: Client::new(),
            get_headers,
            post_headers,
        }
    }
}

#[allow(dead_code)]
impl HttpClient {
    pub async fn get_request<R>(&self, url: &str) -> Result<Option<R>, reqwest::Error>
    where
        R: DeserializeOwned,
    {
        let response = self
            .client
            .get(url)
            .headers(self.get_headers.clone())
            .send()
            .await?;

        self.handle_response(response).await
    }

    pub async fn put_request<T, R>(
        &self,
        url: &str,
        object: &T,
    ) -> Result<Option<R>, reqwest::Error>
    where
        T: Serialize,
        R: DeserializeOwned,
    {
        let response = self
            .client
            .put(url)
            .headers(self.post_headers.clone())
            .body(serde_json::to_string(object).unwrap())
            .send()
            .await?;

        self.handle_response(response).await
    }

    async fn handle_response<T>(
        &self,
        response: reqwest::Response,
    ) -> Result<Option<T>, reqwest::Error>
    where
        T: DeserializeOwned,
    {
        if response.status() == reqwest::StatusCode::OK {
            Ok(Some(serde_json::from_str(&response.text().await?).unwrap()))
        } else {
            Ok(None)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::{Deserialize, Serialize};
    use crate::utils::constants::TESTNET_RPC_ENDPOINT;
    #[derive(Debug, Serialize, Deserialize)]
    struct TestData {
        id: i32,
        name: String,
    }

    #[tokio::test]
    async fn test_get_request() {
        let client = HttpClient::default();
        let result = client.get_request::<TestData>(TESTNET_RPC_ENDPOINT).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_put_request() {
        let client = HttpClient::default();
        let test_data = TestData {
            id: 1,
            name: "test".to_string(),
        };
        let result = client
            .put_request::<TestData, TestData>(TESTNET_RPC_ENDPOINT, &test_data)
            .await;
        assert!(result.is_ok());
    }
}
