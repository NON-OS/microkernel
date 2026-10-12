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

//! Waiting. The capsule's only way to sleep is a receive on its inbox with
//! a timeout; it serves no requests, so whatever lands there is dropped.

use nonos_libc::mk_ipc_recv_from;

const INBOX: u64 = 0;

pub fn wait_ms(ms: u64) {
    let (mut buf, mut from) = ([0u8; 64], 0u32);
    let _ = mk_ipc_recv_from(INBOX, buf.as_mut_ptr(), buf.len(), ms, &mut from);
}

/// About a millisecond: what the HCI transport asks for between two empty
/// looks at the controller.
pub fn pause() {
    wait_ms(1);
}
