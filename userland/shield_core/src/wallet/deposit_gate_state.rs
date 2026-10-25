//! The pool state a deposit is checked against, as read for one depositor and asset.

/// The pool state for one depositor and asset. `open_deposits` lets anyone in, caps still apply.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct PoolState {
    pub deposits_paused: bool,
    pub wound_down: bool,
    pub beta_mode: bool,
    pub beta_paused: bool,
    pub is_depositor: bool,
    pub addr_deposited: u128,
    pub addr_cap: u128,
    pub total_deposited: u128,
    pub total_cap: u128,
    pub fee_bps: u16,
    pub open_deposits: bool,
    /// Base units per note unit. One on a format 5 pool, which counts base units.
    pub scale: u128,
    /// The pool's AmountPolicy is paused: nothing settles in or out until it resumes.
    pub policy_paused: bool,
}
