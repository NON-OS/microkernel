//! The pools before the production pool, kept so their notes stay readable.

use super::super::asset::{ETH, FORMAT5_NOX, NOX};
use super::super::asset_v2::{V2_ETH, V2_NOX};
use super::super::pool::{Pool, Shape};

/// The format 5 pool: composition recomputed on chain, value conserved over the integers.
pub const FORMAT5: Pool = Pool {
    address: "0xDaB92dCcEB48636e07848550858031cEaee3086f",
    deploy_block: 11_764_494,
    words_per_intent: 11,
    shape: Shape::Format5,
    assets: &[FORMAT5_NOX],
    landers: &[],
    policy: None,
    registry: None,
};

/// The launch pool: twelve words per intent, NOX in units of 10^9, no settler, route 2 only.
pub const LAUNCH: Pool = Pool {
    address: "0x8e377752C8890E23A1E9F40eBbD41183Fc6949e2",
    deploy_block: 11_772_152,
    words_per_intent: 12,
    shape: Shape::Launch,
    assets: &[ETH, NOX],
    landers: &[],
    policy: None,
    registry: None,
};

/// The v2 pool: format 7 proofs in the shared form, the v2 image, standard amounts and one fee.
pub const V2: Pool = Pool {
    address: "0xD0dBCe195c082DA39a218C62c01a732CE5b4d541",
    deploy_block: 11_786_912,
    words_per_intent: 12,
    shape: Shape::Launch,
    assets: &[V2_ETH, V2_NOX],
    landers: &[],
    policy: None,
    registry: Some("0xf6b5c3470eb7f1bde3412e72eff4235a4536a206"),
};
