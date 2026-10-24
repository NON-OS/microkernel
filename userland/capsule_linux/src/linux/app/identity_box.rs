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

//! A program's identity inside a `linux --box` box: the services both grant,
//! and its streams held to the box's destinations as well as its own.

use nonos_linux_policy::mask::{narrow, NET_ANON};

use super::identity::AppIdentity;
use super::reach::{chain, ANYWHERE};

impl AppIdentity {
    pub fn boxed(&self, mask: u64, dests: &'static [u8]) -> Self {
        let mask = narrow(self.mask, mask);
        let own = match dests.is_empty() {
            true => ANYWHERE,
            false => [dests, &[]],
        };
        let (mask, reach) = match chain(&self.reach, &own) {
            Some(reach) => (mask, reach),
            None => (mask & !NET_ANON, ANYWHERE),
        };
        Self { mask, reach, ..*self }
    }
}
