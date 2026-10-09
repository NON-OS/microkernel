//! A published spend as a screen follows it: where it went, what the chain shows, and what comes
//! next, every time in whole minutes.

use crate::ffi::evm::to_hex;
use crate::wallet::publish::{Next, Published};

/// Where the spend proved last was published, or why the lander refused it.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Publication {
    pub route: String,
    pub id: Option<String>,
    pub times: u32,
    pub refusal: Option<String>,
}

/// One look at a published spend: waiting, republished, landed with its settlement, or spent by
/// another proof, and whether the owner is now offered to settle it.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct SpendFollow {
    pub state: String,
    pub tx: Option<String>,
    pub link: Option<String>,
    pub minutes_since_first: u32,
    pub times_published: u32,
    pub self_settle_offered: bool,
    pub refusal: Option<String>,
}

/// One spend kept before the last, by the number it is kept under, as `follow_earlier` saw it.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct KeptFollow {
    pub id: String,
    pub follow: SpendFollow,
}

/// A spend no lander holds a record for: `state` says what became of it.
pub(super) fn unpublished(state: &str) -> SpendFollow {
    SpendFollow {
        state: state.into(),
        tx: None,
        link: None,
        minutes_since_first: 0,
        times_published: 0,
        self_settle_offered: state == "unpublished",
        refusal: None,
    }
}

pub(super) fn publication(record: &Published, refusal: Option<String>) -> Publication {
    Publication { route: "lander".into(), id: record.id.clone(), times: record.times, refusal }
}

pub(super) fn follow(record: &Published, now: u64, step: Next, offered: bool) -> SpendFollow {
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
