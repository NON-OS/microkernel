//! What the chain calls hand a screen: a sync, a deposit, and what can be taken back.

use crate::store::Balance;

/// One pass over the pool's history: the head block, what changed, the new balances.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ChainSummary {
    pub head: u64,
    pub received: u32,
    pub deposited: u32,
    pub spent: u32,
    pub balances: Vec<Balance>,
    /// Notes to land and minutes to pass before every note is ready to spend. Zero when ready.
    pub wait_leaves: u32,
    pub wait_minutes: u32,
}

/// A deposit checked against the live pool, approval first. With `refusal` set nothing
/// was kept, since the pool would refuse or trap it. Else the note secret is on disk.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct DepositTicket {
    pub refusal: Option<String>,
    pub amount: String,
    pub fee: String,
    pub note: String,
    pub approve_to: String,
    pub approve_data: String,
    pub deposit_to: String,
    pub deposit_data: String,
    pub value_wei: String,
}

/// What an address could collect and take back from beta, as text past a u64.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Recoverable {
    pub claimable: String,
    pub refundable: String,
}

pub(crate) fn format_wide(units: u128) -> String {
    const SCALE: u128 = 1_000_000_000_000_000_000;
    let whole = units.checked_div(SCALE).unwrap_or(0);
    let fraction = units.checked_rem(SCALE).unwrap_or(0);
    if fraction == 0 {
        return whole.to_string();
    }
    let text = format!("{whole}.{fraction:018}");
    text.trim_end_matches('0').to_string()
}

/// A sealed spend for a relayer: its four hand-off files and the defaults it gave up.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct SpendTicket {
    pub handoff_files: Vec<String>,
    pub weakened: Vec<String>,
}
