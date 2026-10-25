//! Every pool the wallet has known on Sepolia, from the format 5 pool to the production pool.

#[path = "pools_retired.rs"]
mod retired;

use super::asset_v2::{PROD_ETH, PROD_NOX};
use super::pool::{Pool, Shape};
pub use retired::{FORMAT5, LAUNCH, V2};

/// The production landers, tried in this order, each given 20 seconds. The first was confirmed by a
/// transfer that settled through it, and each serves the production pool with the same API.
#[cfg(not(target_vendor = "nonos"))]
const LANDERS: [&str; 5] = [
    "mforujillfk4w5h2zqgxdzendn3qb2zes4r2h57ownojshhmtcdzgdyd.onion",
    "lfpw5uoslfiqmwsc7o2d2qts3grmfnixrhae3hkgkv6x4dlxpvafp4yd.onion",
    "7ywwruseitmycfjwh2upbecetm6edej7pkxpp4xtzmtioes4ubs63kad.onion",
    "g7uyxmffgim7sneprrkdp4ecshupvd3kuaadho6f7e2gcd3gazimriyd.onion",
    "ewjq3ue43pclh6qyx3triup7org64nykbjgy7tmderlzzx27kstfdiqd.onion",
];
/// On NONOS a lander is reached as its Anyone onion service, `<56 letters>.anyone`, beside its Tor
/// onion; no network NONOS runs reaches a Tor onion. These are the four standbys, at 60, 120, 180
/// and 240 s into each root slot, in the STARK lane's order; the primary joins first
/// when its machine is back. Each still has to name this pool in /v1/info and take the
/// submitter's fee before it is handed a proof, and a handoff is followed on the lander that took
/// it. anon_ntor_proofs holds every one to the .anyone checksum.
#[cfg(target_vendor = "nonos")]
const LANDERS: [&str; 4] = [
    "iywfqrj6xyqey574vjtljswxhzeoeqfvlmpqfueva4stooiu3blo7sqd.anyone",
    "5swdytwdtcspbntuofrknlqa7ls2nw2zol2o6tts2jv5hs67n4ee5nad.anyone",
    "ywhgp672zlndlpjubaq23yhbuy526335jfgl2zr74emhgoepovh56lid.anyone",
    "wusdfhdpumeamiyex3v625zled37m2lw24x4bxr3mngu75ryz7vurvad.anyone",
];
pub const LANDER_ON: bool = true;

/// The production pool: 37-limb proofs with a not-before time on a ten-minute grid, and a fee of
/// the protocol part plus one rung of the gas ladder, both read from its policy.
pub const PRODUCTION: Pool = Pool {
    address: "0xaEe51E82965Ec1DeD870F3f4c248Ad4AdDc3e1cb",
    deploy_block: 11_817_433,
    words_per_intent: 13,
    shape: Shape::Launch,
    assets: &[PROD_ETH, PROD_NOX],
    landers: if LANDER_ON { &LANDERS } else { &[] },
    policy: Some("0x660f66ab31Ca9919D9e1770FEDc88Ff2dd29CE59"),
    registry: Some("0xf6b5c3470eb7f1bde3412e72eff4235a4536a206"),
};
