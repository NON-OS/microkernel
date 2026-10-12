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

use super::temp::milli_celsius;
use crate::load::Aml;
use crate::namespace::{Id, Seg};
use crate::platform::{integer, Eval};
use alloc::vec::Vec;

pub const MAX_ACTIVE: usize = 10;

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Trips {
    pub critical: Option<i32>,
    pub hot: Option<i32>,
    pub passive: Option<i32>,
    /// In m°C, _AC0 first. The list stops at the first _ACx missing or
    /// unbelievable, or warmer than the one before: the levels must fall.
    pub active: Vec<i32>,
    /// _TZP, in tenths of a second; 0 means the zone notifies instead.
    pub polling: Option<u64>,
}

pub(super) fn value(aml: &mut Aml, eval: &mut Eval<'_>, tz: Id, seg: Seg) -> Option<u64> {
    let id = aml.ns.child(tz, seg)?;
    eval(aml, id, Vec::new()).ok().as_ref().and_then(integer)
}

fn temp(aml: &mut Aml, eval: &mut Eval<'_>, tz: Id, seg: Seg) -> Option<i32> {
    value(aml, eval, tz, seg).and_then(milli_celsius)
}

pub fn active_seg(x: usize) -> Seg {
    [b'_', b'A', b'C', b'0' + x as u8]
}

pub fn trips(aml: &mut Aml, eval: &mut Eval<'_>, tz: Id) -> Trips {
    let mut active = Vec::new();
    for x in 0..MAX_ACTIVE {
        match temp(aml, eval, tz, active_seg(x)) {
            Some(t) if active.last().is_none_or(|prev| t <= *prev) => active.push(t),
            _ => break,
        }
    }
    Trips {
        critical: temp(aml, eval, tz, *b"_CRT"),
        hot: temp(aml, eval, tz, *b"_HOT"),
        passive: temp(aml, eval, tz, *b"_PSV"),
        active,
        polling: value(aml, eval, tz, *b"_TZP"),
    }
}

/// _TMP, the zone's temperature now, in m°C.
pub fn temperature(aml: &mut Aml, eval: &mut Eval<'_>, tz: Id) -> Option<i32> {
    temp(aml, eval, tz, *b"_TMP")
}
