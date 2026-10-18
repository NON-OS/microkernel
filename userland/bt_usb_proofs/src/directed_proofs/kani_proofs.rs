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

//! Kani harnesses for the directed-advertising check: for every target
//! address and key, an address that is not resolvable private is never
//! taken as this host's, and without a key nothing is.

use crate::link::aimed::aimed_here;
use nonos_bt_wire::addr::BdAddr;

#[kani::proof]
fn only_a_resolvable_private_target_is_ever_ours() {
    let target: [u8; 6] = kani::any();
    let irk: [u8; 16] = kani::any();
    // The top two bits 0b01 mark a resolvable private address (Vol 6,
    // Part B, 1.3.2.2); every other pattern must be refused.
    kani::assume(target[5] >> 6 != 0b01);
    assert!(!aimed_here(&BdAddr(target), Some(&irk)));
}

#[kani::proof]
fn without_a_key_no_target_is_ours() {
    let target: [u8; 6] = kani::any();
    assert!(!aimed_here(&BdAddr(target), None));
}
