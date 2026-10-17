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

//! _AC0 to _AC9 and _TZP under cuts and lies (F-127b), as `trip_read` reads
//! _CRT, _HOT and _PSV. An _ACx unread keeps the value believed before at
//! its level, so neither it nor any cooler level is dropped with its fans;
//! with none believed the list stops there, as at a level absent or warmer
//! than the one before. A _TZP unread keeps the last period. Each sets its
//! bit: said once, and the trips read again at each reading until they read.

use super::trip_read::{read, value, Read, UNREAD_CRT, UNREAD_HOT, UNREAD_PSV};
use super::trips::{active_seg, MAX_ACTIVE};
use crate::load::Aml;
use crate::namespace::Id;
use crate::platform::Eval;
use alloc::vec::Vec;

pub const UNREAD_AC: u16 = 1 << 8;
pub const UNREAD_TZP: u16 = 1 << 9;
/// Every trip whose unread value is kept and read again.
pub const UNREAD_TRIPS: u16 = UNREAD_CRT | UNREAD_HOT | UNREAD_PSV | UNREAD_AC | UNREAD_TZP;

/// _AC0 first, in m°C, falling; `prev`, the list believed before.
pub fn active(
    aml: &mut Aml,
    eval: &mut Eval<'_>,
    tz: Id,
    prev: &[i32],
    faults: &mut u16,
) -> Vec<i32> {
    let mut out: Vec<i32> = Vec::new();
    for x in 0..MAX_ACTIVE {
        let t = match read(aml, eval, tz, active_seg(x)) {
            Read::Absent => break,
            Read::At(t) => Some(t),
            Read::Unread => {
                *faults |= UNREAD_AC;
                prev.get(x).copied()
            }
        };
        match t {
            Some(t) if out.last().is_none_or(|p| t <= *p) => out.push(t),
            _ => break,
        }
    }
    out
}

/// _TZP in tenths of a second; `prev`, the period believed before.
pub fn polling(
    aml: &mut Aml,
    eval: &mut Eval<'_>,
    tz: Id,
    prev: Option<u64>,
    faults: &mut u16,
) -> Option<u64> {
    aml.ns.child(tz, *b"_TZP")?;
    value(aml, eval, tz, *b"_TZP").or_else(|| {
        *faults |= UNREAD_TZP;
        prev
    })
}
