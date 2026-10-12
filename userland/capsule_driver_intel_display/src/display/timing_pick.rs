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

//! The timing a MODE_SET to `w` x `h` is lit at: among the monitor's
//! timings of that size (nonos_modes resolving those EDID names without
//! detail), its preferred one, else the one closest to 60 Hz. And the
//! timing firmware lit, found among them by its actives and totals: the
//! transcoder holds no pixel clock, and putting a mode back needs one.

use nonos_edid::{Detailed, Edid};
use nonos_modes::resolve;

pub fn pick(blob: &[u8], w: u32, h: u32) -> Option<Detailed> {
    let edid = Edid::parse(blob).ok()?;
    let mut best: Option<(bool, u32, Detailed)> = None;
    edid.for_each_timing(&mut |t| {
        if (t.width, t.height) != (w, h) || t.interlaced {
            return;
        }
        let Some(d) = resolve(&t) else { return };
        let off = d.refresh_mhz().abs_diff(60_000);
        let better = match best {
            None => true,
            Some((pref, o, _)) => (t.preferred && !pref) || (t.preferred == pref && off < o),
        };
        if better {
            best = Some((t.preferred, off, d));
        }
    });
    best.map(|b| b.2)
}

/// The monitor's timing with `t`'s actives and totals, clock included.
pub fn matching(blob: &[u8], t: &Detailed) -> Option<Detailed> {
    let edid = Edid::parse(blob).ok()?;
    let key = |d: &Detailed| (d.h_active, d.v_active, d.h_total(), d.v_total());
    let mut found = None;
    edid.for_each_timing(&mut |e| {
        let d = resolve(&e).filter(|d| key(d) == key(t) && !d.interlaced);
        found = found.or(d);
    });
    found
}
