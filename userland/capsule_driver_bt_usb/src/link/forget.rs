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

//! A controller given back is told to Reset (Vol 4, Part E, 7.3.2) before
//! its slot is freed, so it keeps nothing of this capsule's: Reset ends
//! every link it holds, and with them their keys, and stops any scan. The
//! wait is the poll budget's. A controller whose USB transfers already
//! fail is past telling; what such a one still holds is the kernel's to
//! clear when it hands the device to the next holder.

use nonos_bt_hci::{HciError, Host, Transport};
use nonos_bt_wire::hci::opcode as op;

pub fn forget<T: Transport>(host: &mut Host<T>, why: &HciError) {
    if !matches!(why, HciError::Transport(_)) {
        let _ = host.command(op::RESET, &[]);
    }
}
