//! The rewards link: a mainnet address that holds or locks NOX names one testnet address in the
//! registry on Sepolia, and the testnet rewards count its stake there. The mainnet address signs an
//! EIP-712 message and the testnet address sends it, so both consent. A link or a change counts
//! from the next weekly epoch, and a mainnet address changes its link at most once an epoch.

mod calls;
mod check;
mod contract;
mod read;
mod signature;
mod typed;
mod typed_json;

pub use calls::{link_calldata, unlink_calldata, EPOCH, GENESIS, REGISTRY, REGISTRY_BYTES};
pub use check::{counts_from, may_link, may_unlink, LinkRefusal};
pub use contract::accepts;
#[cfg(feature = "fuzzing")]
pub(crate) use read::link_state;
pub use read::{read_link, LinkState};
pub use signature::{parse_signature, sign_link, signer};
pub use typed::digest;
pub use typed_json::typed_json;

#[cfg(test)]
#[path = "check_test.rs"]
mod check_test;
#[cfg(test)]
#[path = "rewards_calls_test.rs"]
mod rewards_calls_test;
#[cfg(test)]
#[path = "rewards_live.rs"]
mod rewards_live;
#[cfg(test)]
#[path = "rewards_test.rs"]
mod rewards_test;
#[cfg(test)]
#[path = "rewards_vectors.rs"]
mod rewards_vectors;
