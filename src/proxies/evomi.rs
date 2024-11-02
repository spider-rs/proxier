use reqwest::header::{CONTENT_TYPE, COOKIE};
use reqwest::{
    header::{HeaderMap, HeaderValue},
    Client,
};
use serde_json::json;
use std::env;

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Default)]
struct ProxtIP {
    /// The local ip address of the server.
    ip_address: String,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Default)]
pub struct ProxyIP {
    /// The ip address of the request
    pub ip_address: String,
    /// The id of the whitelist
    pub id: u128,
    #[serde(default)]
    /// The created at date
    pub created_at: Option<String>,
    #[serde(default)]
    /// The last used data.
    pub last_used_at: Option<String>,
}

// setup any of the proxies white listing if needed.
pub async fn setup_proxy(client: &Client, target: &str, remove: bool) -> ProxyIP {
    let mut p = ProxyIP::default();

    if !target.is_empty() {
        let proxy_url = "https://api.evomi.com/products/ip_whitelist";

        match env::var("EVOMI_API_TOKEN") {
            Ok(password) => {
                let mut headers = HeaderMap::new();

                match HeaderValue::from_str(&format!("Authorization={}", password).to_string()) {
                    Ok(hv) => {
                        headers.insert(COOKIE, hv);
                        headers.insert(
                            CONTENT_TYPE,
                            HeaderValue::from_static("application/json; charset=utf-8"),
                        );
                    }
                    _ => (),
                }

                let action = if remove {
                    client.delete(proxy_url).json(&json!({ "ip": target }))
                } else {
                    client.post(proxy_url).json(&json!({ "ip": target }))
                };

                match action.headers(headers).send().await {
                    Ok(response) => {
                        if response.status().is_success() {
                            if !remove {
                                let ip: ProxyIP = serde_json::from_str(
                                    &response.text().await.unwrap_or_default(),
                                )
                                .unwrap_or_default();

                                p.clone_from(&ip);
                            }
                            println!("Successfully updated the IP authorization for Evomi.");
                        } else {
                            if response.status() == 400 {
                                if !remove {
                                    let res_text = response.text().await;

                                    let ip: ProxyIP =
                                        serde_json::from_str(&res_text.unwrap_or_default())
                                            .unwrap_or_default();

                                    p.clone_from(&ip);
                                }
                                println!("IP already whitelisted for Evomi.");
                            } else {
                                if !remove && response.status() == 500 {
                                    println!(
                                        "IP Already whitelisted for evomi. Status: {:?} - Removed:{:?}",
                                        response.status(),
                                        remove,
                                    );
                                } else {
                                    println!(
                                        "Failed to update the IP authorization. Status: {:?} - Removed:{:?}",
                                        response.status(),
                                        remove,
                                    );
                                }
                            }
                        }
                    }
                    Err(err) => {
                        println!("{:?}", err)
                    }
                }
            }
            Err(e) => {
                println!("Set the env {:?} to enable proxy whitelisting Evomi.", e)
            }
        };
    }

    p
}
