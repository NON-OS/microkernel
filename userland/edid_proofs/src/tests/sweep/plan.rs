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

//! Timings built to be sound within one encoding's field widths, and the
//! test's own statement of soundness, written apart from the crate's.

use super::rng::Rng;
use super::widths::Widths;
use crate::base::dtd::Detailed;

/// The clock and `[ha, hb, hf, hs, va, vb, vf, vs]`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Plan {
    pub khz: u32,
    pub t: [u32; 8],
}

impl Plan {
    /// What a decoded timing says, in a plan's terms.
    pub fn of(d: &Detailed) -> Plan {
        let t = [d.h_active, d.h_blank, d.h_front, d.h_sync];
        let v = [d.v_active, d.v_blank, d.v_front, d.v_sync];
        Plan { khz: d.pixel_khz, t: [t[0], t[1], t[2], t[3], v[0], v[1], v[2], v[3]] }
    }

    pub fn total(&self) -> u64 {
        (self.t[0] + self.t[1]) as u64 * (self.t[4] + self.t[5]) as u64
    }

    /// Sound, as this test reads the rules: a clock, a picture, syncs,
    /// porch and pulse inside each blanking, and 1 Hz to 1 kHz.
    pub fn scans(&self) -> bool {
        let [ha, hb, hf, hs, va, vb, vf, vs] = self.t.map(u64::from);
        let total = self.total();
        let mhz = if total == 0 { 0 } else { self.khz as u64 * 1_000_000 / total };
        let picture = self.khz > 0 && ha > 0 && va > 0 && hs > 0 && vs > 0;
        picture && hf + hs <= hb && vf + vs <= vb && (1_000..=1_000_000).contains(&mhz)
    }
}

/// A sound plan, rolled until one fits the widths.
pub fn plan(r: &mut Rng, w: &Widths) -> Plan {
    loop {
        let (ha, va) = (r.sized(1, w.active), r.sized(1, w.active));
        let (hs, vs) = (r.sized(1, w.h_sync.min(w.blank)), r.sized(1, w.v_sync.min(w.blank)));
        let hf = r.sized(w.front_min, w.h_front.min(w.blank - hs));
        let vf = r.sized(w.front_min, w.v_front.min(w.blank - vs));
        let t = [ha, r.sized(hf + hs, w.blank), hf, hs, va, r.sized(vf + vs, w.blank), vf, vs];
        let mut p = Plan { khz: 0, t };
        let (lo, hi) = (p.total().div_ceil(1_000), p.total().min(w.clock_max_khz as u64));
        p.khz = (r.range(lo as u32, hi as u32) / w.clock_step_khz) * w.clock_step_khz;
        if p.khz >= lo as u32 && p.khz > 0 && p.scans() {
            return p;
        }
    }
}
