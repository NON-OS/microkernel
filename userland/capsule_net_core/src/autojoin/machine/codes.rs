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
//! What a join's result code comes to. The codes are the drivers'
//! (`nonos_wifi_client::join_text`): 0 joined; -6 to -11 the passphrase or
//! the network's security (a handshake that did not finish, which is also
//! how a wrong WPA2 passphrase looks; WPA3 offered no longer; a security this
//! station does not run; a passphrase of the wrong length; WPA3 refusing the
//! password; a message 3 that did not match the beacon), which the same join
//! would meet again; -13 a rejoin the driver refused because the person left
//! the link; anything else may pass (not heard, refused association, keys not
//! installed, no randomness, no answer).

use super::Step;

/// The same join would be turned down again, or must not be tried again.
pub fn refused(code: i32) -> bool {
    matches!(code, -11..=-6 | -13)
}

/// The step a join of saved network `index` that came back with `code` is.
pub fn step_for(index: u8, code: i32) -> Step {
    match code {
        0 => Step::Answered { index, joined: true },
        c if refused(c) => Step::Refused(index),
        _ => Step::Answered { index, joined: false },
    }
}
