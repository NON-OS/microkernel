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

//! Who may reach the lock screen's verifier. Setup chooses the first
//! passphrase; the shell changes it, checks it, keeps its blob and reads
//! its standing. Apart from `callers`, whose names each own a share of the
//! key store: these two own no keys.

use nonos_lock::wire::{OP_LOCK_CHECK, OP_LOCK_LOAD, OP_LOCK_SET, OP_LOCK_STATE};

const SHELL: &[u8] = b"desktop_shell";
const SETUP: &[u8] = b"app.setup_wizard";

pub fn lock_callers(op: u16) -> Option<&'static [&'static [u8]]> {
    match op {
        OP_LOCK_SET => Some(&[SHELL, SETUP]),
        OP_LOCK_CHECK | OP_LOCK_LOAD | OP_LOCK_STATE => Some(&[SHELL]),
        _ => None,
    }
}

/// Whether `sender_pid` may ask lock operation `op`; false for any other op.
pub fn lock_may_ask(op: u16, sender_pid: u32, owner_of: impl Fn(&[u8]) -> Option<u32>) -> bool {
    match lock_callers(op) {
        None => false,
        Some(names) => sender_pid != 0 && names.iter().any(|n| owner_of(n) == Some(sender_pid)),
    }
}
