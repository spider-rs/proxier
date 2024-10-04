use reqwest::Client;
use serde_json::json;
use std::env;
pub use string_concat::{string_concat, string_concat_impl};

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Default)]
pub struct WhitelistEntry {
    /// the id hash.
    pub hash: String,
    /// The server ip
    pub ip: String,
    /// the port
    pub port: u16,
    /// the type
    r#type: String,
    /// the configuration
    pub configuration: String,
}

/// Create a new whitelist entry
pub async fn create_whitelist_entry(
    client: &Client,
    residential_user_hash: &str,
    ip: &str,
    port: u16,
    configuration: &str,
) -> WhitelistEntry {
    let proxy_whitelist_url = string_concat!(
        "https://resi-api.iproyal.com/v1/residential-users/",
        residential_user_hash,
        "/whitelist-entries"
    );

    match dotenv::var("IP_ROYALE_API_TOKEN") {
        Ok(api_token) => {
            let body = json!({
                "ip": ip,
                "port": port,
                "configuration": configuration,
            });

            match client
                .post(&proxy_whitelist_url)
                .bearer_auth(api_token)
                .json(&body)
                .send()
                .await
            {
                Ok(response) => {
                    if response.status().is_success() {
                        response.json().await.unwrap_or_default()
                    } else {
                        if response.status() == 409 {
                            // attempt to get the whitelist entry.
                            println!("Failed to create whitelist entry: {}", response.status());
                            Default::default()
                        } else {
                            // IF THIS FAILS DISABLE IP ROYALE FROM THE SERVER TEMP OR SHUTDOWN AND RESTART
                            println!(
                                "CRITICAL: Failed to create whitelist entry: {}",
                                response.status()
                            );
                            Default::default()
                        }
                    }
                }
                Err(err) => {
                    panic!("Request error: {:?}", err);
                }
            }
        }
        Err(e) => {
            panic!("API Token missing: {:?}", e);
        }
    }
}

/// Get a new whitelist entry
pub async fn get_whitelist_entry(
    residential_user_hash: &str,
    whitelist_entry_hash: &str,
) -> WhitelistEntry {
    let client = Client::new();
    let url = string_concat!(
        "https://resi-api.iproyal.com/v1/residential-users/",
        residential_user_hash,
        "/whitelist-entries/",
        whitelist_entry_hash
    );

    match env::var("IP_ROYALE_API_TOKEN") {
        Ok(api_token) => match client.get(&url).bearer_auth(api_token).send().await {
            Ok(response) => {
                if response.status().is_success() {
                    response.json().await.unwrap_or_default()
                } else {
                    println!("Failed to get whitelist entry: {}", response.status());
                    Default::default()
                }
            }
            Err(err) => {
                panic!("Request error: {:?}", err);
            }
        },
        Err(e) => {
            panic!("API Token missing: {:?}", e);
        }
    }
}

/// Get all the whitelist entries
pub async fn get_whitelist_entries(
    residential_user_hash: &str,
    page: Option<u32>,
    per_page: Option<u32>,
) -> Vec<WhitelistEntry> {
    let client = Client::new();
    let url = string_concat!(
        "https://resi-api.iproyal.com/v1/residential-users/",
        residential_user_hash,
        "/whitelist-entries"
    );

    let mut query_params = vec![];
    if let Some(p) = page {
        query_params.push(("page", p.to_string()));
    }
    if let Some(pp) = per_page {
        query_params.push(("per_page", pp.to_string()));
    }

    match dotenv::var("IP_ROYALE_API_TOKEN") {
        Ok(api_token) => {
            match client
                .get(&url)
                .bearer_auth(api_token)
                .query(&query_params)
                .send()
                .await
            {
                Ok(response) => {
                    if response.status().is_success() {
                        response.json().await.unwrap_or_default()
                    } else {
                        println!("Failed to get whitelist entries: {}", response.status());
                        Default::default()
                    }
                }
                Err(err) => {
                    panic!("Request error: {:?}", err);
                }
            }
        }
        Err(e) => {
            panic!("API Token missing: {:?}", e);
        }
    }
}

/// Update the whitelist entry
pub async fn update_whitelist_entry(
    residential_user_hash: &str,
    whitelist_entry_hash: &str,
    configuration: &str,
) -> WhitelistEntry {
    let client = Client::new();
    let url = string_concat!(
        "https://resi-api.iproyal.com/v1/residential-users/",
        residential_user_hash,
        "/whitelist-entries/",
        whitelist_entry_hash
    );

    match dotenv::var("IP_ROYALE_API_TOKEN") {
        Ok(api_token) => {
            let body = json!({
                "configuration": configuration
            });

            match client
                .put(&url)
                .bearer_auth(api_token)
                .json(&body)
                .send()
                .await
            {
                Ok(response) => {
                    if response.status().is_success() {
                        response.json().await.unwrap_or_default()
                    } else {
                        println!("Failed to update whitelist entry: {}", response.status());
                        Default::default()
                    }
                }
                Err(err) => {
                    panic!("Request error: {:?}", err);
                }
            }
        }
        Err(e) => {
            panic!("API Token missing: {:?}", e);
        }
    }
}

/// Delete the whitelist entry
pub async fn delete_whitelist_entry(
    client: &Client,
    residential_user_hash: &str,
    whitelist_entry_hash: &str,
) {
    let url = string_concat!(
        "https://resi-api.iproyal.com/v1/residential-users/",
        residential_user_hash,
        "/whitelist-entries/",
        whitelist_entry_hash
    );

    match env::var("IP_ROYALE_API_TOKEN") {
        Ok(api_token) => match client.delete(&url).bearer_auth(api_token).send().await {
            Ok(response) => {
                if response.status().is_success() {
                    println!("Whitelist entry deleted successfully.");
                } else {
                    println!("Failed to delete whitelist entry: {}", response.status());
                }
            }
            Err(err) => {
                println!("Request error: {:?}", err);
            }
        },
        Err(e) => {
            panic!("API Token missing: {:?}", e);
        }
    }
}
