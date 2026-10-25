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
//! Whether a message may be taken as from one named peer. Any capsule can
//! send to the shell's inbox, so a key event or a window notice is believed
//! only from the process the registry names for that service.

/// `sender`: who sent it (0 is nobody). `known`: the peer's pid as last read.
/// `fresh`: the registry's answer now, asked only when `known` does not
/// match, so a peer that restarted under a new pid is followed. Returns
/// whether to believe the message, and the pid to remember.
pub fn from_peer(sender: u32, known: u32, fresh: impl FnOnce() -> u32) -> (bool, u32) {
    if sender == 0 {
        return (false, known);
    }
    if sender == known {
        return (true, known);
    }
    let now = fresh();
    (now != 0 && now == sender, now)
}
