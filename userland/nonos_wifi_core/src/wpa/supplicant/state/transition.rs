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

//! The Transition Disable indication, as message 3 carried it.

use super::*;

impl Supplicant {
    /// The Transition Disable KDE's first octet, if message 3 carried one
    /// in key data that verified and unwrapped under this handshake's PTK.
    /// Nothing else sets it: not message 1, a group key message, or a
    /// retransmitted message 3.
    pub fn transition_disable(&self) -> Option<u8> {
        self.transition_disable
    }
}
