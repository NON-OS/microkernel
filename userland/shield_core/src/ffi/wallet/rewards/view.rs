//! The rewards link as a screen shows it: the link in force, and a link or an unlink reviewed.

use crate::evm::checksummed;
use crate::wallet::rewards::{counts_from, LinkRefusal, LinkState};

/// Where the active account stands in the registry. It is the testnet side of any link it makes.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct RewardsLink {
    pub testnet: String,
    /// The mainnet address it is linked to, and whether that address signs as a contract wallet.
    pub mainnet: Option<String>,
    pub contract_wallet: bool,
    /// The epoch the link counts from, the epoch now, and whether the programme has started.
    pub from_epoch: u64,
    pub epoch: u64,
    pub started: bool,
    /// The epoch a link or an unlink made now counts from, and its start in seconds since 1970.
    pub next_epoch: u64,
    pub next_epoch_at: u64,
}

/// A link or an unlink worked out and waiting for a yes, confirmed with `confirm_public_send`.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct RewardsReview {
    /// The review to confirm, or 0 with a refusal.
    pub id: u64,
    pub refusal: Option<String>,
    pub mainnet: String,
    pub testnet: String,
    pub counts_from_epoch: u64,
    pub max_network_fee: String,
    pub valid_for_seconds: u32,
}

pub(super) fn link_view(testnet: &[u8; 20], state: &LinkState) -> RewardsLink {
    let (next_epoch, next_epoch_at) = counts_from(state);
    RewardsLink {
        testnet: checksummed(testnet),
        mainnet: state.linked_to.as_ref().map(checksummed),
        contract_wallet: state.contract_wallet,
        from_epoch: state.from_epoch,
        epoch: state.epoch,
        started: state.started,
        next_epoch,
        next_epoch_at,
    }
}

pub(super) fn sentence(refusal: LinkRefusal) -> &'static str {
    match refusal {
        LinkRefusal::TestnetTaken => {
            "This account is linked to another mainnet address. Unlink it first."
        }
        LinkRefusal::ChangedThisEpoch => {
            "That mainnet address changed its link this epoch. It can change again next epoch."
        }
        LinkRefusal::AlreadyLinked => {
            "That link is already in force. Sending it again would delay it."
        }
        LinkRefusal::NotLinked => "This account is not linked to a mainnet address.",
    }
}
