//! The pools this wallet knows, read from the chain. Pools before format 5 are wound down.

use super::asset::{Asset, Coin, NOX};
pub use super::pools::{FORMAT5, LAUNCH, PRODUCTION, V2};

pub const CHAIN_ID: u64 = 11_155_111;
/// A pool's contract shape. Format 5 takes eleven words in base units with no fee.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Shape {
    Format5,
    Launch,
}

/// One deployed `ShieldedPool`. A scan starts at `deploy_block` and not before.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Pool {
    pub address: &'static str,
    pub deploy_block: u64,
    pub words_per_intent: u8,
    pub shape: Shape,
    pub assets: &'static [Asset],
    pub landers: &'static [&'static str],
    pub policy: Option<&'static str>,
    /// Where a spend's association root must be published, when the pool has a registry.
    pub registry: Option<&'static str>,
}

pub const ACTIVE: Pool = PRODUCTION;

/// `address(1)`: a spend's fee goes to whoever submits its settlement.
pub const SUBMITTER: [u8; 20] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1];

impl Pool {
    pub fn asset(&self, coin: Coin) -> Option<Asset> {
        self.assets.iter().copied().find(|a| a.coin == coin)
    }
}

pub const NOX_ASSET: u64 = NOX.id;
pub const NOTE_COMMITTED: &str =
    "0x4d307fe1adf9127a7d28c596bdfd7feabb9b3ee98ae414268ca26f6429c4c7c7";
pub const OUTPUT_NOTE: &str = "0x4d420d56046a6ee2218a6d109d30db7a92c661255d344cf8c07172a40b4ab926";
pub const NULLIFIER_SPENT: &str =
    "0x2d8b76eb247945151bd870d531c84c5420f53e3cb9c0ab3b350f46bb09362096";
pub const ROOT_COMMITTED: &str =
    "0xb7bfae93aefdea72b2c686d45574222375d73ca1db6f50e1b605a9d5b286f6d1";
/// RPCs tried in turn, each checked to answer a Tor exit. A history with a hole is refused.
/// The Sepolia servers the history is read from, in order. NONOS puts Tenderly's
/// gateway first, which the STARK lane names as one that returns logs whole;
/// a short history is refused against `nextLeafIndex()` on any of them.
pub const RPCS: [&str; 3] = [
    "sepolia.gateway.tenderly.co",
    "rpc.sepolia.ethpandaops.io",
    "ethereum-sepolia-rpc.publicnode.com",
];
