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

//! A controller given back is first told to Reset (Vol 4, Part E, 7.3.2),
//! so it keeps no link, key or scan of this capsule's; one whose USB
//! transfers already fail is not asked.

use crate::fixture::{air, keyboard, Stack};
use crate::link::forget;
use nonos_bt_hci::HciError;
use nonos_bt_wire::hci::opcode as op;

#[test]
fn a_controller_given_back_is_told_to_reset_first() {
    let reset = op::RESET.0.to_le_bytes();
    let resets = |host: &mut Stack| air(host).commands.iter().filter(|c| c[..2] == reset).count();
    let (mut host, ..) = keyboard();
    let before = resets(&mut host);
    forget(&mut host, &HciError::Reset);
    assert_eq!(resets(&mut host), before + 1);
    forget(&mut host, &HciError::Transport(-5));
    assert_eq!(resets(&mut host), before + 1, "a dead transport is not asked");
}
