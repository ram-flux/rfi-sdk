//
//  Copyright 2024 Ram Flux, LLC.
//

use reqwest::blocking::Client as BlockingClient;
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use reqwest::{Client, Method};

use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Deserialize)]
pub struct EmptyResponse {}

#[derive(Debug, Deserialize, Serialize)]
pub struct HttpResponse<T> {
    pub code: u32,
    pub message: String,
    pub result: Option<T>,
}

impl<T> TryFrom<&str> for HttpResponse<T>
where
    T: DeserializeOwned,
{
    type Error = anyhow::Error;
    fn try_from(value: &str) -> anyhow::Result<Self> {
        serde_json::from_str(value)
            .map_err(|e| anyhow::anyhow!("Failed to deserialize from string: {}", e))
    }
}

impl<T: DeserializeOwned> HttpResponse<T> {
    pub async fn from_response_text<U: DeserializeOwned>(
        response_text: &str,
    ) -> anyhow::Result<Option<U>> {
        let response: HttpResponse<Value> = serde_json::from_str(response_text)
            .map_err(|e| anyhow::anyhow!("Failed to deserialize response text: {}", e))?;

        match response.code {
            200 => {
                if let Some(data_value) = response.result {
                    if data_value.is_string() && data_value.to_string().is_empty() {
                        // Special handling for empty string
                        Ok(None)
                    } else {
                        let parsed_data: U = serde_json::from_value(data_value)
                            .map_err(|e| anyhow::anyhow!("Failed to parse data value: {}", e))?;
                        Ok(Some(parsed_data))
                    }
                } else {
                    Ok(None)
                }
            }
            _ => Ok(None),
        }
    }
}

impl<T: DeserializeOwned> HttpResponse<T> {
    pub fn from_response_blocking_text<U: DeserializeOwned>(
        response_text: &str,
    ) -> anyhow::Result<Option<U>> {
        let response: HttpResponse<Value> = serde_json::from_str(response_text).map_err(|e| {
            anyhow::anyhow!(
                "Failed to deserialize response text in blocking call: {}",
                e
            )
        })?;

        match response.code {
            200 => {
                if let Some(data_value) = response.result {
                    if data_value.is_string() && data_value.to_string().is_empty() {
                        // Special handling for empty string
                        Ok(None)
                    } else {
                        let parsed_data: U = serde_json::from_value(data_value).map_err(|e| {
                            anyhow::anyhow!("Failed to parse data value in blocking call: {}", e)
                        })?;
                        Ok(Some(parsed_data))
                    }
                } else {
                    Ok(None)
                }
            }
            _ => Ok(None),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ApiClient {
    pub client: Client,
    pub blocking_client: Option<BlockingClient>,
}

impl ApiClient {
    pub fn new() -> Self {
        ApiClient {
            client: Client::new(),
            blocking_client: None,
        }
    }

    pub fn send_blocking(
        &self,
        method: reqwest::Method,
        url: &str,
        body: Option<String>,
        headers: Option<Vec<(&str, &str)>>,
    ) -> anyhow::Result<String> {
        let client = BlockingClient::new(); //&self.blocking_client;
        let mut request_builder = client.request(method, url);

        if let Some(body_content) = body {
            request_builder = request_builder.body(body_content);
        }

        if let Some(header_list) = headers {
            let mut header_map = HeaderMap::new();
            for (key, value) in header_list {
                let key = HeaderName::from_bytes(key.as_bytes())
                    .map_err(|e| anyhow::anyhow!("Invalid header name: {}", e))?;

                let value = HeaderValue::from_str(value)
                    .map_err(|e| anyhow::anyhow!("Invalid header value: {}", e))?;
                header_map.insert(key, value);
            }
            request_builder = request_builder.headers(header_map);
        }

        let response = request_builder
            .send()
            .map_err(|e| anyhow::anyhow!("Failed to send request in blocking mode: {}", e))?;

        if !response.status().is_success() {
            anyhow::bail!("HTTP request failed with status: {}", response.status());
        }

        let text = response
            .text()
            .map_err(|e| anyhow::anyhow!("Failed to read response text in blocking mode: {}", e))?;
        Ok(text)
    }

    pub async fn send(
        &self,
        method: reqwest::Method,
        url: &str,
        body: Option<String>,
        headers: Option<Vec<(&str, &str)>>,
    ) -> anyhow::Result<String> {
        let client = &self.client;
        let mut request = client.request(method, url);

        if let Some(body_content) = body {
            request = request.body(body_content);
        }

        if let Some(header_list) = headers {
            for (key, value) in header_list {
                request = request.header(key, value);
            }
        }

        let response = request
            .send()
            .await
            .map_err(|e| anyhow::anyhow!("Failed to send request: {}", e))?;

        // tracing::info!("[send_request] response status: {:?}", response.status());
        if !response.status().is_success() {
            anyhow::bail!("HTTP request failed with status: {}", response.status());
        }

        let text = response
            .text()
            .await
            .map_err(|e| anyhow::anyhow!("Failed to read response text: {}", e))?;
        // tracing::info!("[send_request] text: {text}");
        Ok(text)
    }

    pub async fn send_form_data(
        method: Method,
        url: &str,
        form_data: Option<std::collections::HashMap<String, String>>,
        headers: Option<Vec<(&str, &str)>>,
    ) -> anyhow::Result<String> {
        let client = Client::new();
        let mut request = client.request(method, url);

        if let Some(form_data) = form_data {
            request = request.form(&form_data);
        }

        if let Some(header_list) = headers {
            for (key, value) in header_list {
                request = request.header(key, value);
            }
        }

        let mut response = request
            .send()
            .await
            .map_err(|e| anyhow::anyhow!("Failed to send request: {}", e))?;

        // tracing::info!("[send_request] response status: {:?}", response.status());
        if !response.status().is_success() {
            anyhow::bail!("HTTP request failed with status: {}", response.status());
        }

        let mut data = bytes::BytesMut::new();
        while let Some(chunk) = response
            .chunk()
            .await
            .map_err(|e| anyhow::anyhow!("is failed to chunk: {}", e))?
        {
            data.extend_from_slice(&chunk);
        }
        // tracing::info!("[send_request] data: {:?}", data);
        let data_as_string = String::from_utf8(data.to_vec())
            .map_err(|e| anyhow::anyhow!("bytes to string failed: {}", e))?;

        // tracing::info!("[send_request] data_as_string: {}", data_as_string);
        Ok(data_as_string)
    }

    /**
     *  Async version of from_json
     */
    pub async fn from_json<T: DeserializeOwned>(response: &str) -> anyhow::Result<Option<T>> {
        let res = HttpResponse::<EmptyResponse>::from_response_text::<T>(response).await?;
        Ok(res)
    }

    /**
     * Blocking version of from_json_to
     */
    pub fn blocking_from_jsos<T: DeserializeOwned>(response: &str) -> anyhow::Result<Option<T>> {
        let res = HttpResponse::<EmptyResponse>::from_response_blocking_text::<T>(response);
        res
    }
}
