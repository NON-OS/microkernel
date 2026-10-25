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
//!
//! The one widening: a program with no policy of its own, which runs in the
//! owner's default box (nonos_linux_policy::unlisted), is given NET_ANON by a
//! box that names destinations, and is held to exactly that list; so is each
//! such program the run execs (boxed::exec_program). The person who types the
//! command owns the machine and that default; a program with a published
//! policy of its own is only ever narrowed.

use nonos_linux_policy::mask::{narrow, NET_ANON};
use nonos_linux_policy::unlisted;

use super::identity::AppIdentity;
use super::reach::{chain, ANYWHERE};

impl AppIdentity {
    pub fn boxed(&self, mask: u64, dests: &'static [u8]) -> Self {
        let mask = narrow(self.widened(dests).mask, mask);
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

    /// The program's own services, with NET_ANON added only for a program
    /// with no policy of its own in a box that names destinations.
    pub fn widened(&self, dests: &[u8]) -> Self {
        match !dests.is_empty() && self.policy == unlisted(self.policy.program_blake3) {
            true => Self { mask: self.mask | NET_ANON, ..*self },
            false => *self,
        }
    }
}
