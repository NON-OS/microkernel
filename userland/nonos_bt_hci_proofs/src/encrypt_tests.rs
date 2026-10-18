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

//! Encrypting a link as central: LE Enable Encryption (Vol 4, Part E,
//! 7.8.24) and the Encryption Change that answers it (7.7.8).

use crate::sim::link_events::{encrypt_answered, encryption_change};
use crate::sim::stack::stack;
use nonos_bt_hci::host::enable_encryption_params;
use nonos_bt_hci::HciError;
use nonos_bt_wire::hci::opcode as op;
use nonos_bt_wire::WireError;

/// An LTK as the Security Manager hands it over, most significant first.
pub const LTK: [u8; 16] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];

#[test]
fn the_ltk_goes_least_significant_octet_first_after_a_zero_rand_and_ediv() {
    let mut want = vec![0x19, 0x20, 28, 0x40, 0x00, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    want.extend((0..16u8).rev());
    let mut s = stack(encrypt_answered(vec![encryption_change(0, 0x40, 0x01)]));
    assert_eq!(s.encrypt(0x40, &LTK), Ok(()));
    assert_eq!(s.transport().pipes().commands[0].1, want);
    let reserved = Err(HciError::Wire(WireError::Reserved));
    assert_eq!(enable_encryption_params(0x0f00, &LTK), reserved);
}

#[test]
fn another_links_change_is_kept_and_this_links_awaited() {
    let other = encryption_change(0, 0x41, 0x01);
    let mut s = stack(encrypt_answered(vec![other.clone(), encryption_change(0, 0x40, 0x01)]));
    assert_eq!(s.encrypt(0x40, &LTK), Ok(()));
    assert_eq!(s.next_event(), Ok(Some(other)));
}

#[test]
fn a_refusal_or_a_link_left_clear_is_a_status() {
    let start = op::LE_ENABLE_ENCRYPTION;
    // PIN or Key Missing: the peer has lost the bond (Vol 1, Part F, 2).
    let mut s = stack(encrypt_answered(vec![encryption_change(0x06, 0x40, 0x00)]));
    assert_eq!(s.encrypt(0x40, &LTK), Err(HciError::Status(start, 0x06)));
    let mut s = stack(encrypt_answered(vec![encryption_change(0, 0x40, 0x00)]));
    assert_eq!(s.encrypt(0x40, &LTK), Err(HciError::Status(start, 0)));
}
