// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors (AGPL-3.0-or-later)

//! Who may create, import or sign with a key, by op.
//!
//! The kernel admits any IPC holder to the keyring's endpoint, and each
//! handler checks only that the caller owns the key it names. So any capsule
//! could make keys of its own until the store was full, and the wallet's
//! next one answered ENOSPC (m/06). These ops now go only to the names that
//! use them: the wallet's windows, the meter (`payment`) and, for its bond
//! records, the Bluetooth driver. Every other op stays any owner's, asked
//! of its own keys, which a stranger can no longer make.
//!
//! Pure: the caller supplies who owns each name. Held in `wallet_proofs`.

use super::types::*;

const W0: &[u8] = b"app.nonos_wallet";
const W1: &[u8] = b"app.nonos_wallet.1";
const W2: &[u8] = b"app.nonos_wallet.2";
const METER: &[u8] = b"payment";
const BONDS: &[u8] = b"driver.bt_usb0";

/// Every name that may ask an op below. Each owns at most its share of the
/// store, so together they can never fill it for one another.
pub const CREATORS: [&[u8]; 5] = [W0, W1, W2, METER, BONDS];

/// The names that may ask `op`, or `None` for an op any owner may ask.
pub fn callers(op: u16) -> Option<&'static [&'static [u8]]> {
    match op {
        OP_STORE => Some(&[BONDS]),
        OP_WALLET_GENERATE => Some(&[METER]),
        OP_WALLET_IMPORT | OP_SIGN_TX | OP_SIGN_TYPED_DATA => Some(&[W0, W1, W2, METER]),
        OP_WALLET_GENERATE_HD
        | OP_WALLET_RECOVER
        | OP_WALLET_DERIVE
        | OP_SIGN_NOX_STAKE_APPROVE
        | OP_SIGN_NOX_STAKE
        | OP_SIGN_NOX_UNSTAKE
        | OP_SIGN_NOX_STAKE_LOCKED => Some(&[W0, W1, W2]),
        _ => None,
    }
}

/// Whether `sender_pid` may ask `op`, given `owner_of`, the pid that owns a
/// name. Pid 0 is never a named caller.
pub fn may_ask(op: u16, sender_pid: u32, owner_of: impl Fn(&[u8]) -> Option<u32>) -> bool {
    match callers(op) {
        None => true,
        Some(names) => {
            sender_pid != 0 && names.iter().any(|name| owner_of(name) == Some(sender_pid))
        }
    }
}
