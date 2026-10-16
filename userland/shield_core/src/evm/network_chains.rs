//! The two networks' fixed figures: chain ids, RPCs, token contracts, explorers.

use super::super::const_hex::hex20;
use super::Chain;
use crate::net::tor::Purpose;

pub(super) const MAINNET: Chain = Chain {
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
    swaps: Some(super::super::swap::MAINNET),
    shield: false,
    explorer: crate::net::explorer::MAINNET_TX,
    purpose: Purpose::Mainnet,
};

pub(super) const SEPOLIA: Chain = Chain {
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
