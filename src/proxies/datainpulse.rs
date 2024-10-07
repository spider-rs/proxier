use reqwest::Client;
use std::env;

/// Get the user name password.
pub fn get_user_name_password() -> (String, String) {
    (
        env::var("DATA_INPULSE_USERNAME").unwrap_or_default(),
        env::var("DATA_INPULSE_PASSWORD").unwrap_or_default(),
    )
}

/// Create a new whitelist entry
pub async fn create_whitelist_entry(client: &Client, ip: &str) {
    let proxy_whitelist_url = format!("https://gw.dataimpulse.com:777/api/whitelist_ip/{}", ip);

    let (username, password) = get_user_name_password();

    match client
        .post(&proxy_whitelist_url)
        .basic_auth(username, Some(password))
        .send()
        .await
    {
        Ok(response) => {
            if response.status().is_success() {
                let value: serde_json::Value = response.json().await.unwrap_or_default();
                println!("Whitelisting datainpulse: {:?}", value);
            } else {
                if response.status() == 409 {
                    // attempt to get the whitelist entry.
                    println!("Failed to create whitelist entry: {}", response.status());
                    Default::default()
                } else {
                    println!(
                        "CRITICAL: Failed to create whitelist entry: {}",
                        response.status()
                    );
                }
            }
        }
        Err(err) => {
            println!("Request error: {:?}", err);
        }
    }
}

/// Delete the whitelist entry
pub async fn delete_whitelist_entry(client: &Client, ip: &str) {
    let url = format!("https://gw.dataimpulse.com:777/api/whitelist_ip/{}", ip);
    let (username, password) = get_user_name_password();

    match client
        .delete(&url)
        .basic_auth(username, Some(password))
        .send()
        .await
    {
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
    }
}
