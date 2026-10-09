//! The networks the public account lives on, kept apart so nothing for one reaches the other.

use super::const_hex::hex20;
use crate::net::tor::Purpose;

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

const MAINNET: Chain = Chain {
    chain_id: 1,
    rpcs: &[
        "ethereum-rpc.publicnode.com",
        "1.rpc.thirdweb.com",
        "eth.rpc.blxrbdn.com",
        "eth-mainnet.public.blastapi.io",
    ],
    send_rpcs: &["rpc.flashbots.net", "rpc.mevblocker.io"],
    nox_token: hex20("0a26c80be4e060e688d7c23addb92cbb5d2c9eca"),
    nox_implementations: &[hex20("9bf9d1dc66c7c571b59f5a9195f46d2d19cdfb3c")],
    usdc_token: hex20("a0b86991c6218b36c1d19d4a2e9eb0ce3606eb48"),
    swaps: Some(super::swap::MAINNET),
    shield: false,
    explorer: crate::net::explorer::MAINNET_TX,
    purpose: Purpose::Mainnet,
};

const SEPOLIA: Chain = Chain {
    chain_id: 11_155_111,
    rpcs: &[
        "ethereum-sepolia-rpc.publicnode.com",
        "rpc.sepolia.ethpandaops.io",
        "sepolia.rpc.thirdweb.com",
    ],
    send_rpcs: &["ethereum-sepolia-rpc.publicnode.com", "sepolia.rpc.thirdweb.com"],
    nox_token: hex20("3e5249a65ca513d5e11260222e0d26f46b465d36"),
    nox_implementations: &[hex20("72d0509749a01cc44637eb4505dbe69ce740e73e")],
    usdc_token: hex20("1c7d4b196cb0c7b01d743fbc6116a902379c7238"),
    swaps: None,
    shield: true,
    explorer: crate::net::explorer::SEPOLIA_TX,
    purpose: Purpose::Sepolia,
};
