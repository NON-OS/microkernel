//! The networks the public account lives on, kept apart so nothing for one reaches the other.

#[path = "network_chains.rs"]
mod chains;

use crate::net::tor::Purpose;
use chains::{MAINNET, SEPOLIA};

/// A network a screen can select.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum Network {
    Mainnet,
    Sepolia,
}

/// What the wallet knows about one network.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Chain {
    pub chain_id: u64,
    /// HTTPS read RPCs, tried in turn over Tor, each checked to answer a Tor exit and take a full
    /// batch. Tenderly refuses Tor, and dRPC and Nodies cap batches, so none is listed.
    pub rpcs: &'static [&'static str],
    /// Where signed sends go: on mainnet, private relays, so a sale is not sandwiched. Each is
    /// also asked about waiting sends, so a host that refuses some Tor exits serves reads only.
    pub send_rpcs: &'static [&'static str],
    /// The NOX proxy and the checked implementations. Any other stops NOX sends.
    pub nox_token: [u8; 20],
    pub nox_implementations: &'static [[u8; 20]],
    pub usdc_token: [u8; 20],
    /// Where swaps trade, or none on a network with no liquidity for them.
    pub swaps: Option<super::swap::Venue>,
    /// Whether the shielded pool exists here.
    pub shield: bool,
    pub explorer: &'static str,
    /// The circuits reserved for this network's account reads and sends.
    pub purpose: Purpose,
}

impl Chain {
    /// The token contract of `coin` here, or none for ether.
    pub fn token(&self, coin: crate::net::asset::Coin) -> Option<[u8; 20]> {
        use crate::net::asset::Coin;
        match coin {
            Coin::Eth => None,
            Coin::Nox => Some(self.nox_token),
            Coin::Usdc => Some(self.usdc_token),
        }
    }
}

impl Network {
    pub fn chain(self) -> Chain {
        match self {
            Network::Mainnet => MAINNET,
            Network::Sepolia => SEPOLIA,
        }
    }

    /// The folder name for this network's nonce record and caches.
    pub fn slug(self) -> &'static str {
        match self {
            Network::Mainnet => "mainnet",
            Network::Sepolia => "sepolia",
        }
    }
}
