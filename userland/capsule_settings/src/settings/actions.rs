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

//! What an action row does. The lock screen's passphrase row asks the shell
//! to open its own prompt, in its trusted band: the passphrase is typed
//! there, checked by the keyring, and never passes through Settings.

use crate::settings::access::shell::{call, OP_LOCK_PROMPT, OP_LOCK_STANDING};
use crate::settings::schema::rows::Action;
use crate::settings::state::status::StatusKind;
use crate::settings::state::State;

pub fn run(state: &mut State, action: Action) {
    match action {
        Action::LockPassphrase => match call(OP_LOCK_PROMPT, &[], &mut []) {
            Ok(()) => state.status.set(StatusKind::Idle, b"the passphrase prompt is open"),
            Err(_) => state.status.set(StatusKind::Error, b"the desktop did not open the prompt"),
        },
    }
}

/// Whether the shell says a passphrase is set; None when it did not answer.
pub fn lock_set() -> Option<bool> {
    let mut out = [0u8; 4];
    call(OP_LOCK_STANDING, &[], &mut out).ok().map(|()| u32::from_le_bytes(out) == 1)
}
