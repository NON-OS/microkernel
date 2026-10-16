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

//! Which port to ask next, and which are given up on for now.

use super::ports::{Ports, Seen, TRIES};
use crate::xhci::{Port, PORT_CLAIMED, PORT_FREE};

impl Ports {
    /// The first free port in `listed` that may be asked now, and whether any
    /// is still open: to be asked at once, or with a class driver still
    /// classifying its device.
    pub fn next(&self, listed: &[Port]) -> (Option<u8>, bool) {
        let (mut first, mut open) = (None, false);
        for p in listed.iter().filter(|p| p.owner != PORT_CLAIMED) {
            let (due, opens) = match (self.0[usize::from(p.id)], p.owner == PORT_FREE) {
                (Seen::NotOurs, _) => (false, false),
                (Seen::Later { tries, next }, free) if tries >= TRIES => {
                    (free && next.expired(), false)
                }
                (_, false) => (false, true),
                (Seen::New, true) => (true, true),
                (Seen::Later { next, .. }, true) => (next.expired(), true),
            };
            open |= opens;
            first = first.or(due.then_some(p.id));
        }
        (first, open)
    }

    /// The ports in `listed` given up on for now.
    pub fn closed(&self, listed: &[Port]) -> u8 {
        let shut = |p: &&Port| match self.0[usize::from(p.id)] {
            Seen::NotOurs => true,
            Seen::Later { tries, .. } => tries >= TRIES,
            Seen::New => false,
        };
        listed.iter().filter(shut).count().min(u8::MAX as usize) as u8
    }
}
