use crate::chain::Network;
use crate::electrum::discovery::{DiscoveryManager, Service};

pub fn add_default_servers(discovery: &DiscoveryManager, network: Network) {
    match network {
        #[cfg(not(feature = "liquid"))]
        Network::Bitcoin => {
            discovery
                .add_default_server(
                    "electrumx.pozoqo.tech".into(),
                    vec![Service::Tcp(50001), Service::Ssl(50002)],
                )
                .ok();
            discovery
                .add_default_server(
                    "electrumx.pozoqo.tech".into(),
                    vec![Service::Ssl(50002)],
                )
                .ok();
