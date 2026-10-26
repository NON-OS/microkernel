// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors (AGPL-3.0-or-later)

//! The key gate (m/05 section 4, m/06): only the wallet's windows, the meter
//! and the Bluetooth driver create, import or sign, each only by the ops it
//! uses, and together they cannot fill the store.

use crate::server::protocol::callers::{callers, may_ask};
use crate::server::protocol::types::*;

const W: [u32; 3] = [40, 41, 52];
const METER: u32 = 70;
const BONDS: u32 = 80;
const CALCULATOR: u32 = 61;
const HD: [u16; 7] = [
    OP_WALLET_GENERATE_HD,
    OP_WALLET_RECOVER,
    OP_WALLET_DERIVE,
    OP_SIGN_NOX_STAKE_APPROVE,
    OP_SIGN_NOX_STAKE,
    OP_SIGN_NOX_UNSTAKE,
    OP_SIGN_NOX_STAKE_LOCKED,
];
const SHARED: [u16; 3] = [OP_WALLET_IMPORT, OP_SIGN_TX, OP_SIGN_TYPED_DATA];

fn registry(name: &[u8]) -> Option<u32> {
    match name {
        b"app.nonos_wallet" => Some(W[0]),
        b"app.nonos_wallet.1" => Some(W[1]),
        b"app.nonos_wallet.2" => Some(W[2]),
        b"payment" => Some(METER),
        b"driver.bt_usb0" => Some(BONDS),
        b"app.calculator" => Some(CALCULATOR),
        _ => None,
    }
}

/// Who the table lets ask `op`, from the pids above.
fn admitted(op: u16) -> Vec<u32> {
    [W[0], W[1], W[2], METER, BONDS, CALCULATOR, 7]
        .into_iter()
        .filter(|&pid| may_ask(op, pid, registry))
        .collect()
}

#[test]
fn each_row_admits_exactly_its_callers() {
    assert_eq!(admitted(OP_STORE), [BONDS]);
    assert_eq!(admitted(OP_WALLET_GENERATE), [METER], "the wallet makes keys by HD");
    for op in SHARED {
        assert_eq!(admitted(op), [W[0], W[1], W[2], METER], "op {op}");
    }
    for op in HD {
        assert_eq!(admitted(op), W, "op {op}");
    }
}

#[test]
fn a_calculator_can_neither_make_a_key_nor_sign() {
    for op in (0..64).filter(|&op| callers(op).is_some()) {
        assert!(!may_ask(op, CALCULATOR, registry), "op {op}");
    }
    assert_eq!((0..64).filter(|&op| callers(op).is_some()).count(), 12);
}

#[test]
fn pid_0_and_an_absent_name_admit_no_one() {
    for op in [OP_STORE, OP_WALLET_GENERATE, OP_SIGN_TX, OP_WALLET_DERIVE] {
        assert!(!may_ask(op, 0, |_| Some(0)), "op {op}");
        assert!(!may_ask(op, METER, |_| None), "op {op}");
    }
}
