# proxier

The auto white-listing proxy API lib for Rust.

## Services

* WebShare
* IpRoyale
- DataInpulse

`cargo add proxier`

Example

```rust
use proxier::proxies::{Proxier, IPRoyaleConfiguration, WebShareConfiguration};

#[tokio::main]
async fn main() {
    // replace with the server ip
    let mut proxier = Proxier::new("124.32.334.2");

    let iproyale_config = IPRoyaleConfiguration::default();
    let webshare_config = WebShareConfiguration::default();

    // setup all the configs for the proxies.

    proxier.setup_proxies(Some(iproyale_config, webshare_config)).await;

    // whitelist the server
    proxier.whitelist().await;

    // add signals with startup on the server to delist after shutdown using tokio::select etc.

    // delist the proxiers for the server after.
    proxier.delist().await;
}
```

## ENV

The following env variables are required to set.

`PROXY_SHARE_PASSWORD`
`IP_ROYALE_API_TOKEN`