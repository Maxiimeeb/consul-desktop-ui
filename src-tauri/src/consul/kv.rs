use reqwest;
use base64::prelude::*;
use std::sync::LazyLock;
use super::client::ConsulClient;

#[derive(serde::Deserialize)]
struct ReadResponse {
    #[serde(rename = "Key")]
    key: String,
    #[serde(rename = "Value")]
    value: String,
}

static HTTP_CLIENT: LazyLock<reqwest::Client> = LazyLock::new(reqwest::Client::new);

/// Read a key from Consul. If the key does not exist, None will be returned.
pub async fn read(client: &ConsulClient, key: &str) -> Result<Option<String>, Box<dyn std::error::Error>> {
    let url = format!("{}/v1/kv/{}", client.address(), key);
    let response = HTTP_CLIENT.get(url).send().await?;

    if response.status() == 404 {
        return Ok(None);
    }

    if !response.status().is_success() {
        return Err(
            format!(
                "Failed to read key got ({}) with {}", response.status(), response.text().await?
            ).into()
        );
    }

    let json_response: Vec<ReadResponse> = response.json().await?;

    let decoded_value = BASE64_STANDARD.decode(&json_response[0].value)?;

    Ok(Some(String::from_utf8(decoded_value)?))
}

pub async fn list_all_keys(client: &ConsulClient) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let url = format!("{}/v1/kv/{}?keys", client.address(), "/");
    let response = HTTP_CLIENT.get(url).send().await?;

    if !response.status().is_success() {
        return Err(
            format!(
                "Failed to list keys got ({}) with {}", response.status(), response.text().await?
            ).into()
        );
    }

    let keys: Vec<String> = response.json().await?;

    Ok(keys)
}

pub async fn delete(client: &ConsulClient, key: &str) -> Result<(), Box<dyn std::error::Error>> {
    let url = format!("{}/v1/kv/{}", client.address(), key);
    let response = HTTP_CLIENT.delete(url).send().await?;

    if !response.status().is_success() {
        return Err(
            format!(
                "Failed to delete key got ({}) with {}", response.status(), response.text().await?
            ).into()
        );
    }

    Ok(())
}

pub async fn set(client: &ConsulClient, key: &str, value: &str) -> Result<(), Box<dyn std::error::Error>> {
    let url = format!("{}/v1/kv/{}", client.address(), key);
    let response = HTTP_CLIENT.put(url).body(value.to_string()).send().await?;

    if !response.status().is_success() {
        return Err(
            format!(
                "Failed to write key got ({}) with {}", response.status(), response.text().await?
            ).into()
        );
    }

    Ok(())
}