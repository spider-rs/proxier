use iproyale::WhitelistEntry;
use reqwest::Client;
use webshare::ProxyIP;

/// iproyal
pub mod iproyale;
/// webshare proxy
pub mod webshare;

/// The ip royale configuration for the proxy.
#[derive(Default, Clone, Debug)]
pub struct IPRoyaleConfiguration {
    /// The residential user hash
    pub residential_user_hash: String,
    /// The port
    pub port: u16,
    /// The configuration type
    pub configuration: String,
    /// The proxy ip results when whitelisting
    pub whitelist_entry: Option<WhitelistEntry>,
}

/// The ip royale configuration for the proxy.
#[derive(Default, Clone, Debug)]
pub struct WebShareConfiguration {
    /// The proxy ip results when whitelisting
    pub whitelist_entry: Option<ProxyIP>,
}

/// The proxy service you want to use.
#[derive(Default, Clone, Debug)]
pub struct Proxier {
    /// Iproyale service.
    pub iproyale: Option<IPRoyaleConfiguration>,
    /// Webshare service.
    pub webshare: Option<WebShareConfiguration>,
    /// The shared client.
    pub client: Client,
    /// The server ip NAT to whitelist.
    pub server_ip: String,
}

impl Proxier {
    /// A new proxier setup.
    pub fn new(server_ip: &str) -> Proxier {
        Proxier {
            server_ip: server_ip.into(),
            iproyale: None,
            webshare: None,
            client: Client::default(),
        }
    }
    /// Setup all of the proxies needed.
    pub async fn setup_proxies(
        &mut self,
        iproyale: Option<IPRoyaleConfiguration>,
        webshare: Option<WebShareConfiguration>,
    ) {
        if self.server_ip.is_empty() {
            // try to get the ip via webshare or other services.
            self.server_ip = webshare::get_ip(&self.client).await;
        }
        self.iproyale = iproyale;
        self.webshare = webshare;
    }

    /// Whitelist the server ips all at once.
    pub async fn whitelist(&mut self) {
        self.whitelist_webshare().await;
        self.whitelist_iproyale().await;
    }

    /// Whitelist webshare.
    pub async fn whitelist_webshare(&mut self) {
        if let Some(webshare) = self.webshare.as_mut() {
            let proxy_results = webshare::setup_proxy(&self.client, &self.server_ip, false).await;
            let _ = webshare.whitelist_entry.insert(proxy_results);
        }
    }

    /// Whitelist webshare.
    pub async fn whitelist_iproyale(&mut self) {
        if let Some(iproyale) = self.iproyale.as_mut() {
            let proxy_results = iproyale::create_whitelist_entry(
                &self.client,
                &iproyale.residential_user_hash,
                &self.server_ip,
                iproyale.port,
                &iproyale.configuration,
            )
            .await;
            let _ = iproyale.whitelist_entry.insert(proxy_results);
        }
    }

    /// Delist all the proxy entries.
    pub async fn delist(&mut self) {
        self.delist_webshare().await;
        self.delist_iproyale().await;
    }

    /// Delist a webshare entry for whitelisting.
    pub async fn delist_webshare(&mut self) {
        if let Some(webshare) = self.webshare.as_mut() {
            if let Some(whitelist_entry) = webshare.whitelist_entry.as_mut().take() {
                webshare::setup_proxy(&self.client, &whitelist_entry.id.to_string(), true).await;
            }
        }
    }

    /// Delist a iproyale entry for whitelisting.
    pub async fn delist_iproyale(&mut self) {
        if let Some(iproyale) = self.iproyale.as_mut() {
            if let Some(whitelist_entry) = iproyale.whitelist_entry.as_mut().take() {
                iproyale::delete_whitelist_entry(
                    &self.client,
                    &iproyale.residential_user_hash,
                    &whitelist_entry.hash,
                )
                .await
            }
        }
    }
}
