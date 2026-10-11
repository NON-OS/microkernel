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

//! The full timing for a mode, computed one way by every driver: what EDID
//! details is taken as it is; a size and rate alone is the DMT mode where
//! the DMT has one, else CVT 1.2.

use crate::cvt::{cvt, Blanking};
use crate::dmt::dmt;
use nonos_edid::{Detailed, Source, Timing};

/// The timing for `width` x `height` at `refresh_mhz`: the DMT's mode of
/// that size and whole rate with the same kind of blanking, else CVT.
pub fn compute(width: u32, height: u32, refresh_mhz: u32, b: Blanking) -> Option<Detailed> {
    let reduced = b != Blanking::Standard;
    if refresh_mhz.is_multiple_of(1000) {
        if let Some(m) = dmt(width, height, refresh_mhz / 1000, reduced) {
            return Some(m.timing);
        }
    }
    cvt(width, height, refresh_mhz, b)
}

/// The timing for one EDID timing. Established and standard timings name
/// DMT modes (E-EDID 1.4 sections 3.8 and 3.9); a CVT code names a CVT
/// timing (section 3.10.3.11); interlaced modes and VICs without their
/// detail have none here.
pub fn resolve(t: &Timing) -> Option<Detailed> {
    if let Some(d) = t.detail {
        return Some(d);
    }
    if t.interlaced {
        return None;
    }
    let b = if t.reduced_blanking { Blanking::Reduced } else { Blanking::Standard };
    match t.source {
        Source::Established | Source::Standard => compute(t.width, t.height, t.refresh_mhz, b),
        Source::Cvt => cvt(t.width, t.height, t.refresh_mhz, b),
        _ => None,
    }
}
