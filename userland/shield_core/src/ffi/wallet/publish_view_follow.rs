//! One look at a published spend, from its record and the step it is at.

use super::SpendFollow;
use crate::ffi::evm::to_hex;
use crate::wallet::publish::{Next, Published};

pub(in crate::ffi::wallet) fn follow(
    record: &Published,
    now: u64,
    step: Next,
    offered: bool,
) -> SpendFollow {
    let tx = match step {
        Next::Landed(Some(tx)) => Some(to_hex(&tx)),
        _ => None,
    };
    let state = match step {
        Next::Landed(_) => "landed",
        Next::Elsewhere => "spent elsewhere",
        Next::Wait => "waiting",
        Next::Republish => "republished",
    };
    let explorer = crate::evm::Network::Sepolia.chain().explorer;
    SpendFollow {
        state: state.into(),
        link: tx.as_ref().map(|t| format!("{explorer}{t}")),
        tx,
        minutes_since_first: u32::try_from(
            now.saturating_sub(record.first).checked_div(60).unwrap_or(0),
        )
        .unwrap_or(u32::MAX),
        times_published: record.times,
        self_settle_offered: offered,
        refusal: None,
    }
}
