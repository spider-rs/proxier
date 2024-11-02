# proxier

The auto white-listing proxy API lib for Rust.

## Services

* [WebShare](https://www.webshare.io/)
* [DataInpulse](https://dataimpulse.com/)
* [Evomi](https://evomi.com/)
* [IpRoyale](https://iproyal.com/)

`cargo add proxier`

Example

```rust
use proxier::proxies::{Proxier, IPRoyaleConfiguration, WebShareConfiguration, DatainpulseConfiguration, EvomiConfiguration};

#[tokio::main]
async fn main() {
    // replace with the server ip
    let mut proxier = Proxier::new("124.32.334.2");

    let iproyale_config = IPRoyaleConfiguration::default();
    let webshare_config = WebShareConfiguration::default();
    let datainpulse_config = DatainpulseConfiguration::default();
    let evomi_config = EvomiConfiguration::default();

    // setup all the configs for the proxies.

    proxier.setup_proxies(Some(iproyale_config), Some(webshare_config), Some(datainpulse_config), Some(evomi_config)).await;

    // whitelist the server
    proxier.whitelist().await;

    // add signals with startup on the server to delist after shutdown using tokio::select etc.

    // delist the proxiers for the server after.
    proxier.delist().await;
}
```

## ENV

The following env variables are required to set.

### Webshare

`PROXY_SHARE_PASSWORD`

### IPRoyale

`IP_ROYALE_API_TOKEN`

### Datainpulse

`DATA_INPULSE_USERNAME`
`DATA_INPULSE_PASSWORD`

### Evomi

`EVOMI_API_TOKEN`