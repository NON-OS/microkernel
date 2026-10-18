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

//! A trip point read when the firmware may be cut short or lie (F-127).
//! An object that is there but whose answer was cut by the budget, is not
//! an integer, or is outside the believable range (`temp`) is unread, not
//! absent: a trip dropped would read the zone as cooler than it is, so the
//! value last believed is kept, and the capsule says so once per zone.

use super::temp::milli_celsius;
use crate::load::Aml;
use crate::namespace::{Id, Seg};
use crate::platform::{integer, Eval};
use alloc::vec::Vec;

/// `Trips::faults` bits beside `trip_rule`'s: the trip was unread.
pub const UNREAD_CRT: u16 = 1 << 4;
pub const UNREAD_HOT: u16 = 1 << 5;
pub const UNREAD_PSV: u16 = 1 << 6;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Read {
    Absent,
    Unread,
    /// In m°C.
    At(i32),
}

pub(super) fn value(aml: &mut Aml, eval: &mut Eval<'_>, tz: Id, seg: Seg) -> Option<u64> {
    let id = aml.ns.child(tz, seg)?;
    eval(aml, id, Vec::new()).ok().as_ref().and_then(integer)
}

pub(super) fn temp(aml: &mut Aml, eval: &mut Eval<'_>, tz: Id, seg: Seg) -> Option<i32> {
    value(aml, eval, tz, seg).and_then(milli_celsius)
}

pub fn read(aml: &mut Aml, eval: &mut Eval<'_>, tz: Id, seg: Seg) -> Read {
    if aml.ns.child(tz, seg).is_none() {
        return Read::Absent;
    }
    temp(aml, eval, tz, seg).map_or(Read::Unread, Read::At)
}

/// The value believed: the one read, or for an unread trip the one kept,
/// with `bit` set in `faults`.
pub fn believe(r: Read, kept: Option<i32>, bit: u16, faults: &mut u16) -> Option<i32> {
    match r {
        Read::Absent => None,
        Read::At(t) => Some(t),
        Read::Unread => {
            *faults |= bit;
            kept
        }
    }
}
