use crate::variants::network::Network;

use super::exceptions::NetworkException;

pub fn get_network_url(network: Network) -> String {
    match network {
        Network::Test => "https://testnet.xrpl-labs.com/".to_string(),
        Network::Dev => "https://s.devnet.rippletest.net:51234/".to_string(),
        Network::XahauTest => "https://xahau-test.net/".to_string(),
        Network::SidechainTest => "https://sidechain-net2.devnet.rippletest.net:51234/".to_string(),
        Network::Custom(url) => url,
    }
}

pub fn get_network_uri(network: Network) -> String {
    match network {
        Network::Test => "wss://testnet.xrpl-labs.com/".to_string(),
        Network::Dev => "wss://s.devnet.rippletest.net:51233/".to_string(),
        Network::XahauTest => "wss://xahau-test.net/".to_string(),
        Network::SidechainTest => "wss://sidechain-net2.devnet.rippletest.net:51233/".to_string(),
        Network::Custom(url) => url,
    }
}

pub fn get_faucet_network_url(network: Network) -> Result<String, NetworkException> {
    match network {
        Network::Test => Ok("https://faucet.altnet.rippletest.net/accounts".to_string()),
        Network::Dev => Ok("https://faucet.devnet.rippletest.net/accounts".to_string()),
        Network::XahauTest => Ok("https://xahau-test.net/accounts".to_string()),
        Network::SidechainTest => Err(NetworkException::NoFaucetUrl(Network::SidechainTest)),
        Network::Custom(url) => Ok(url),
    }
}
