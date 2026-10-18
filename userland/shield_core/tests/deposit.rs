//! A deposit in the form the live pool takes it.
//!
//! The call is pinned against Foundry's own ABI encoder, the fee against the
//! contract's rounding, the stored note against the watcher that must later
//! recognise it, and the pre-flight against the contract's refusals in order.

#[path = "deposit/launch.rs"]
mod launch;
#[path = "deposit/note.rs"]
mod note;
#[path = "deposit/preflight.rs"]
mod preflight;

use nox_shield_core::custody::{generate_phrase, phrase_to_seed};
use nox_shield_core::keys::Account;
use nox_shield_core::wallet::PoolState;

const NOX: u64 = 1_000_000_000_000_000_000;
const NOX_ASSET: u64 = 1;

fn account() -> Account {
    let phrase = generate_phrase().expect("entropy");
    Account::from_seed(&phrase_to_seed(&phrase).expect("seed")).expect("account")
}

fn hex(b: &[u8]) -> String {
    b.iter().map(|x| format!("{x:02x}")).collect()
}

fn open_pool() -> PoolState {
    PoolState {
        beta_mode: true,
        is_depositor: true,
        addr_cap: 10 * u128::from(NOX),
        total_cap: 100 * u128::from(NOX),
        fee_bps: 25,
        scale: 1,
        ..PoolState::default()
    }
}
