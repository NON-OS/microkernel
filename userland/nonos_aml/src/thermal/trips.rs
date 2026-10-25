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

//! A thermal zone's trip points (ACPI 6.5, 11.4): _CRT, where the machine
//! must shut down; _HOT, where it should sleep to S4; _PSV, where the
//! processors slow; and _AC0 to _AC9, where fans start, _AC0 the hottest.

use super::trip_more::{active, polling};
use super::trip_read::{believe, read, temp, UNREAD_CRT, UNREAD_HOT, UNREAD_PSV};
use super::trip_rule::sane;
use crate::load::Aml;
use crate::namespace::{Id, Seg};
use crate::platform::Eval;
use alloc::vec::Vec;

pub const MAX_ACTIVE: usize = 10;

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Trips {
    pub critical: Option<i32>,
    pub hot: Option<i32>,
    pub passive: Option<i32>,
    /// In m°C, _AC0 first. The list stops at the first _ACx missing, or
    /// warmer than the one before, or unread with none kept (`trip_more`).
    pub active: Vec<i32>,
    /// _TZP, in tenths of a second; 0 means the zone notifies instead.
    pub polling: Option<u64>,
    /// `trip_rule`'s fault bits, how the trips were misordered, and
    /// `trip_read`'s and `trip_more`'s, which were unread and kept from `prev`.
    pub faults: u16,
}

pub fn active_seg(x: usize) -> Seg {
    [b'_', b'A', b'C', b'0' + x as u8]
}

/// The zone's trips; `prev`, those believed before, for any unread now.
pub fn trips(aml: &mut Aml, eval: &mut Eval<'_>, tz: Id, prev: &Trips) -> Trips {
    let mut kept = 0;
    let active = active(aml, eval, tz, &prev.active, &mut kept);
    let mut get = |seg, was, bit| believe(read(aml, eval, tz, seg), was, bit, &mut kept);
    let critical = get(*b"_CRT", prev.critical, UNREAD_CRT);
    let passive = get(*b"_PSV", prev.passive, UNREAD_PSV);
    let (hot, faults) = sane(critical, get(*b"_HOT", prev.hot, UNREAD_HOT), passive);
    let polling = polling(aml, eval, tz, prev.polling, &mut kept);
    let faults = u16::from(faults) | kept;
    Trips { critical, hot, passive, active, polling, faults }
}

/// _TMP, the zone's temperature now, in m°C.
pub fn temperature(aml: &mut Aml, eval: &mut Eval<'_>, tz: Id) -> Option<i32> {
    temp(aml, eval, tz, *b"_TMP")
}
