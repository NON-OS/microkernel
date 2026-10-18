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

//! What a zone's _CRT, _HOT and _PSV are believed as when firmware orders
//! them wrongly (the lead's F-110b ruling). _CRT is never dropped: a
//! power-off too early costs availability on the wiped, data-safe path,
//! while a real _CRT dropped can cost hardware with no trip of its own.
//! _HOT, the weaker trip, is dropped only when a _CRT still guards the
//! zone above where it stood. Each fault is a bit, for the capsule to say
//! once per zone. No floor, and no comparison with a first reading.

/// _HOT at or above _CRT: dropped, _CRT is the stronger trip.
pub const HOT_OVER_CRT: u8 = 1 << 0;
/// _HOT at or below _PSV with a _CRT above _PSV: dropped, _CRT guards.
pub const HOT_UNDER_PSV: u8 = 1 << 1;
/// _HOT at or below _PSV with no _CRT above _PSV: kept, said.
pub const HOT_UNDER_PSV_KEPT: u8 = 1 << 2;
/// _CRT at or below _PSV: kept, said; the lie stays on the protective side.
pub const CRT_UNDER_PSV: u8 = 1 << 3;

/// The _HOT believed, and the faults found, from the three trips as read.
pub fn sane(crt: Option<i32>, hot: Option<i32>, psv: Option<i32>) -> (Option<i32>, u8) {
    let crt_low = matches!((crt, psv), (Some(c), Some(p)) if c <= p);
    let guarded = matches!((crt, psv), (Some(c), Some(p)) if c > p);
    let mut faults = if crt_low { CRT_UNDER_PSV } else { 0 };
    let hot = match hot {
        Some(h) if crt.is_some_and(|c| h >= c) => {
            faults |= HOT_OVER_CRT;
            None
        }
        Some(h) if psv.is_some_and(|p| h <= p) && guarded => {
            faults |= HOT_UNDER_PSV;
            None
        }
        Some(h) if psv.is_some_and(|p| h <= p) => {
            faults |= HOT_UNDER_PSV_KEPT;
            Some(h)
        }
        h => h,
    };
    (hot, faults)
}
