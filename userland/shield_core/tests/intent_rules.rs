//! The spend rules, each against the pool's own refusal.

#[path = "intent_rules/balance.rs"]
mod balance;
#[path = "intent_rules/fee.rs"]
mod fee;
#[path = "intent_rules/relay.rs"]
mod relay;

use nox_shield_core::wallet::{Intent, PoolRules};

/// The format 5 pool: eleven words, no relay fee.
const FORMAT5: PoolRules = PoolRules { words_per_intent: 11, max_relay_fee: 0, policy_fee: None };
/// A launch pool: twelve words, transfers may pay a relayer up to 1,000 units.
const LAUNCH: PoolRules =
    PoolRules { words_per_intent: 12, max_relay_fee: 1_000, policy_fee: None };
const RELAYER: [u8; 20] = [9; 20];

const NOX: u64 = 1_000_000_000_000_000_000;
const A: u64 = 1;

fn transfer() -> Intent {
    // 3 NOX in, 2 to the payee, 1 back as change.
    Intent {
        inputs: [(2 * NOX, A, 10), (NOX, A, 11)],
        outputs: [(2 * NOX, A), (NOX, A)],
        public_amount: 0,
        fee: 0,
        asset_id: A,
        clearing_price: 0,
        recipient: [0; 20],
        fee_recipient: [0; 20],
    }
}

fn unshield(fee: u64) -> Intent {
    // 3 NOX in, 1 leaves the pool to a recipient, fee taken from it, rest as change.
    let public_amount = 1_000_000;
    Intent {
        inputs: [(2_000_000, A, 10), (1_000_000, A, 11)],
        outputs: [(2_000_000 - fee, A), (0, A)],
        public_amount,
        fee,
        asset_id: A,
        clearing_price: 0,
        recipient: [7; 20],
        fee_recipient: [0; 20],
    }
}
