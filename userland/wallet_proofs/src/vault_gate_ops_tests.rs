// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! Which ops the vault gate is asked of (W3): the vault's and the shield's
//! seal and open, and the recovery words, which owner-checking alone left to
//! any capsule that came to own a words entry. The dispatcher asks the gate
//! before any op is served.

use crate::server::protocol::gated::wallet_only;
use crate::server::protocol::types::*;

#[test]
fn the_words_and_the_vault_are_the_wallets_alone() {
    let gated = [OP_VAULT_SEAL, OP_VAULT_OPEN, OP_SHIELD_MATERIAL, OP_SHIELD_SEAL, OP_SHIELD_OPEN];
    for op in gated {
        assert!(wallet_only(op), "op {op} is asked of the gate");
    }
    // The meter exports and signs with its own key, so these stay owner-checked.
    for op in [OP_WALLET_EXPORT, OP_SIGN_TX, OP_SIGN_TYPED_DATA, OP_WALLET_ADDRESS, OP_STORE] {
        assert!(!wallet_only(op), "op {op} is any owner's");
    }
}

#[test]
fn the_dispatcher_asks_the_gate_before_any_op_is_served() {
    let path = concat!(env!("CARGO_MANIFEST_DIR"), "/../capsule_keyring/src/server/dispatch.rs");
    let src = std::fs::read_to_string(path).expect("dispatch.rs");
    let gate = src.find("wallet_only(req.op) && !super::vault_gate::allowed(sender_pid)");
    let served = src.find("match req.op");
    assert!(gate.is_some() && gate < served, "the gate comes first in dispatch");
}
