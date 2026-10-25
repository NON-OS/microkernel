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

//! A receipt's block field, and the idle lock.

#[test]
fn a_receipts_block_is_read_from_its_own_field() {
    use crate::wallet::rpc::parse_receipt_block;
    let r = br#"{"id":100,"result":{"blockNumber":"0x15f2a3c","status":"0x1"}}"#;
    assert_eq!(parse_receipt_block(r), Some(0x15f2a3c));
    assert_eq!(parse_receipt_block(br#"{"id":100,"result":null}"#), None);
    assert_eq!(parse_receipt_block(br#"{"result":{"blockNumber":"0x"}}"#), None);
}

/* A window left alone locks after five minutes, never while a payment goes. */
#[test]
fn an_idle_window_locks_itself_but_never_mid_payment() {
    use crate::idle_lock::{due, IDLE_LOCK_MS};
    assert!(!due(IDLE_LOCK_MS - 1, 0, false, false));
    assert!(due(IDLE_LOCK_MS, 0, false, false));
    assert!(!due(IDLE_LOCK_MS * 3, 0, true, false), "already locked");
    assert!(!due(IDLE_LOCK_MS * 3, 0, false, true), "a payment is going out");
}

/* The one bound every nonce read or signed is held to (EIP-2681). */
#[test]
fn a_nonce_of_u64_max_is_never_believed() {
    use crate::sent::usable;
    assert_eq!(usable(u64::MAX), None, "a node that says u64::MAX is not believed");
    assert_eq!(usable(u64::MAX - 1), Some(u64::MAX - 1));
    assert_eq!(usable(0), Some(0));
}
