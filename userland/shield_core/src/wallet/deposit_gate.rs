//! Whether the pool would take a deposit, with the contract's checks in its order.

use super::deposit::split_deposit;

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
}

/// Why the pool would refuse, named the way a screen can say it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Refusal {
    DepositsPaused,
    /// The pool never settles again, so money deposited could leave only by refund.
    WoundDown,
    /// Zero, above one note's ceiling, or a fee that would take all of it.
    Amount,
    /// Not a whole number of pool units. The remainder would back no note.
    NotWholeUnits,
    BetaPaused,
    NotOnBetaList,
    AddressCapReached,
    PoolCapReached,
}

/// The contract's `absorb` checks, in its order, for `amount` in base units.
pub fn check(state: &PoolState, amount: u128) -> Result<(), Refusal> {
    if state.deposits_paused {
        return Err(Refusal::DepositsPaused);
    }
    if state.wound_down {
        return Err(Refusal::WoundDown);
    }
    let units = units_of(amount, state.scale)?;
    if state.beta_mode {
        if state.beta_paused {
            return Err(Refusal::BetaPaused);
        }
        if !state.open_deposits && !state.is_depositor {
            return Err(Refusal::NotOnBetaList);
        }
        // The caps count what was sent, in base units, not the note's units.
        if state.addr_deposited.checked_add(amount).is_none_or(|t| t > state.addr_cap) {
            return Err(Refusal::AddressCapReached);
        }
        if state.total_deposited.checked_add(amount).is_none_or(|t| t > state.total_cap) {
            return Err(Refusal::PoolCapReached);
        }
    }
    split_deposit(units, state.fee_bps).map(|_| ()).ok_or(Refusal::Amount)
}

/// Base units as whole note units: at least one, below p - 1, nothing left over.
pub fn units_of(amount: u128, scale: u128) -> Result<u64, Refusal> {
    let units = amount.checked_div(scale).ok_or(Refusal::Amount)?;
    if amount.checked_rem(scale) != Some(0) {
        return Err(Refusal::NotWholeUnits);
    }
    match u64::try_from(units) {
        Ok(u) if u > 0 && u <= crate::notes::MAX_VALUE => Ok(u),
        _ => Err(Refusal::Amount),
    }
}
