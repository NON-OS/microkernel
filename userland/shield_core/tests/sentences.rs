// Tests assert by panicking, so the lints that forbid it are off here.
#![allow(clippy::expect_used, clippy::panic)]

use nox_shield_core::error::{CustodyError, WalletError};

// Every error a shell can be handed, listed exhaustively. Each shell maps an error to a
// sentence with a fallback, so a new core variant would silently read "Something went wrong".
// These matches have no wildcard arm: a new variant stops this file compiling, the reminder
// that both shells need a sentence for it. The checks here happen at compile time.

/// The name each variant goes by in the shells' tables.
fn named(error: &WalletError) -> &'static str {
    match error {
        WalletError::Custody { .. } => "custody",
        WalletError::Store { .. } => "store",
        WalletError::Net { .. } => "net",
        WalletError::Prove { .. } => "prove",
        WalletError::Insufficient => "insufficient",
        WalletError::NoSpendPlan => "no spend plan",
        WalletError::Address => "address",
        WalletError::Amount => "amount",
        WalletError::ReviewExpired => "review expired",
        WalletError::NoSuchAccount => "no such account",
        WalletError::Unavailable => "unavailable",
        WalletError::NotStandard => "not standard",
        WalletError::FeeTooHigh => "fee too high",
        WalletError::TooSoon => "too soon",
        WalletError::FeeChanged => "fee changed",
    }
}

fn named_custody(error: &CustodyError) -> &'static str {
    match error {
        CustodyError::Entropy => "entropy",
        CustodyError::Mnemonic => "mnemonic",
        CustodyError::PhraseLength => "phrase length",
        CustodyError::SealAuth => "seal auth",
        CustodyError::Guard => "guard",
        CustodyError::VaultShape => "vault shape",
        CustodyError::VaultPresent => "vault present",
        CustodyError::Locked => "locked",
        CustodyError::KeyShape => "key shape",
    }
}

#[test]
fn every_wallet_error_has_a_name_a_shell_can_look_up() {
    let all = [
        WalletError::Custody { source: CustodyError::Locked },
        WalletError::Store { source: nox_shield_core::error::StoreError::RowShape },
        WalletError::Net { source: nox_shield_core::error::NetError::Transport },
        WalletError::Prove { source: nox_shield_core::error::ProveError::Cancelled },
        WalletError::Insufficient,
        WalletError::NoSpendPlan,
        WalletError::Address,
        WalletError::Amount,
        WalletError::ReviewExpired,
        WalletError::NoSuchAccount,
        WalletError::Unavailable,
        WalletError::NotStandard,
        WalletError::FeeTooHigh,
        WalletError::TooSoon,
        WalletError::FeeChanged,
    ];
    for error in &all {
        assert!(!named(error).is_empty(), "a variant reached the shells unnamed");
    }
}

#[test]
fn every_custody_error_has_a_name_a_shell_can_look_up() {
    let all = [
        CustodyError::Entropy,
        CustodyError::Mnemonic,
        CustodyError::PhraseLength,
        CustodyError::SealAuth,
        CustodyError::Guard,
        CustodyError::VaultShape,
        CustodyError::VaultPresent,
        CustodyError::Locked,
        CustodyError::KeyShape,
    ];
    for error in &all {
        assert!(!named_custody(error).is_empty(), "a variant reached the shells unnamed");
    }
}
