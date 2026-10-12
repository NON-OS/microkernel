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
//! The clock a USB Audio 2.0 stream runs on: its terminal's bCSourceID in
//! the AudioControl interface, a Clock Source, or a Clock Selector whose
//! first input is one (2.0 4.7.2.1 to 4.7.2.5, as recalled, unverified).
//! A clock multiplier, or a selector before anything else, is not followed.

use super::entities::{entities, Entity};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Clock {
    /// The AudioControl interface the clock's requests go to.
    pub interface: u8,
    pub id: u8,
    pub programmable: bool,
    /// A selector before it, set to its first input at bind.
    pub selector: Option<u8>,
}

/// The clock `terminal` runs on, in the 2.0 function of `config`.
pub fn clock(config: &[u8], terminal: u8) -> Option<Clock> {
    let (interface, seen) = entities(config);
    let source = |want: u8| {
        seen.iter().find_map(|e| match *e {
            Entity::Source { id, programmable } if id == want => Some((id, programmable)),
            _ => None,
        })
    };
    let at = seen.iter().find_map(|e| match *e {
        Entity::Terminal { id, clock } if id == terminal => Some(clock),
        _ => None,
    })?;
    let (selector, (id, programmable)) = match source(at) {
        Some(s) => (None, s),
        None => seen.iter().find_map(|e| match *e {
            Entity::Selector { id, first } if id == at => Some((Some(id), source(first)?)),
            _ => None,
        })?,
    };
    Some(Clock { interface, id, programmable, selector })
}
