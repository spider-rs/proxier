use reqwest::{
    header::{HeaderMap, HeaderValue, AUTHORIZATION},
    Client,
};
use serde_json::json;
pub use string_concat::{string_concat, string_concat_impl};

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
        let proxy_share_url = "https://proxy.webshare.io/api/v2/proxy/ipauthorization/";

        match dotenv::var("PROXY_SHARE_PASSWORD") {
            Ok(password) => {
                let mut headers = HeaderMap::new();

                match HeaderValue::from_str(
                    &string_concat!("Token ".to_string(), password).to_string(),
                ) {
                    Ok(hv) => {
                        headers.insert(AUTHORIZATION, hv);
                    }
                    _ => (),
                }

                let action = if remove {
                    client.delete(string_concat!(proxy_share_url, target, "/"))
                } else {
                    client
                        .post(proxy_share_url)
                        .json(&json!({ "ip_address": target }))
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
                            println!("Successfully updated the IP authorization.");
                        } else {
                            if response.status() == 400 {
                                if !remove {
                                    // we need to get the existing IP from the system to get the ID.
                                    // we first need to list all of the IPS from the proxy until we find our ID. After finding our ID we can remove the value.

                                    let res_text = response.text().await;

                                    let ip: ProxyIP =
                                        serde_json::from_str(&res_text.unwrap_or_default())
                                            .unwrap_or_default();

                                    p.clone_from(&ip);
                                }
                                println!("IP already whitelisted.");
                            } else {
                                println!(
                                    "Failed to update the IP authorization. Status: {:?} - Removed:{:?}",
                                    response.status(),
                                    remove,
                                );
                            }
                        }
                    }
                    Err(err) => {
                        println!("{:?}", err)
                    }
                }
            }
            Err(e) => {
                println!("Set the env {:?} to enable proxy whitelisting.", e)
            }
        };
    }

    p
}

// setup any of the proxies white listing if needed.
pub async fn get_local_ip(client: &Client) -> String {
    // set the whitelist to the proxy service.
    let metadata_url = "https://api.ipify.org";
    let mut ip_address = String::new();

    match client.get(metadata_url).send().await {
        Ok(response) => {
            if response.status().is_success() {
                ip_address = response.text().await.unwrap_or_default();
                println!("The IP address of the local server is {}", ip_address);
            } else {
                eprintln!(
                    "Failed to retrieve IP address. Status: {}",
                    response.status()
                );
            }
        }
        Err(e) => {
            println!("{:?}", e)
        }
    }

    ip_address
}

// Get the main local IP address
pub async fn get_ip(client: &Client) -> String {
    let proxy_share_url = "https://proxy.webshare.io/api/v2/proxy/ipauthorization/whatsmyip/";
    let mut ip_address = String::new();

    // SETUP IP whitelisting. TODO: move this to another file.
    match dotenv::var("PROXY_SHARE_PASSWORD") {
        Ok(password) => {
            let mut headers = HeaderMap::new();

            match HeaderValue::from_str(&string_concat!("Token ".to_string(), password).to_string())
            {
                Ok(hv) => {
                    headers.insert(AUTHORIZATION, hv);
                }
                _ => (),
            }

            match client.get(proxy_share_url).headers(headers).send().await {
                Ok(response) => {
                    if response.status().is_success() {
                        let text = response.text().await.unwrap_or_default();
                        let ip: ProxtIP = serde_json::from_str(&text).unwrap_or_default();

                        println!("The IP address of the server is {}", ip.ip_address);

                        ip_address = ip.ip_address;
                    } else {
                        eprintln!(
                            "Failed to retrieve IP address. Status: {}",
                            response.status()
                        );

                        ip_address = get_local_ip(&client).await;
                    }
                }
                Err(e) => {
                    println!("{:?}", e);
                    ip_address = get_local_ip(&client).await;
                }
            }
        }
        Err(e) => {
            println!("Set the env {:?} to enable proxy whitelisting.", e)
        }
    };

    ip_address
}
