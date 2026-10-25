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

//! Which input a link may post. Keys come only from a bond protected from
//! a device in the middle. Passkey Entry needs the user's own hands on the
//! keyboard (Vol 3, Part H, 2.3.5.3); Just Works needs no one at all
//! (2.3.5.2). Any device in range that pairs that way and calls itself a
//! keyboard could otherwise type into this machine. Pointer motion and
//! buttons are still posted from such a link: a mouse has no way to take
//! part in pairing, so it can only ever pair by Just Works.

use super::env::Env;
use super::session::Session;
use nonos_bt_hogp::report::Input;

impl Session {
    /// Whether keys from this link reach the input ring.
    pub(super) fn may_type(&self) -> bool {
        self.bond.is_some_and(|b| b.authenticated)
    }

    pub(super) fn post<E: Env>(&self, input: Input, env: &mut E) {
        let key = matches!(input, Input::KeyDown(_) | Input::KeyUp(_));
        if self.may_type() || !key {
            // The router learns the device's level from the event (G-76); a
            // key only ever reaches here from an authenticated bond.
            env.input(input, self.bond.is_some_and(|b| b.authenticated));
        }
    }
}
