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

//! The plan a takeover follows (G-72): each lit output's monitor read
//! once, the GGTT entries this driver's windows will take, and which lit
//! surfaces are firmware's. A lit surface reaching into those entries, or
//! into the upper half of the GGTT, is a dead driver's leftover, not
//! firmware's picture: it is never copied (a client's pixels may be in it,
//! or, its frames freed under it, another capsule's) and the windows are
//! mapped over it, so the plane shows the new, zeroed surfaces at once.

use nonos_intel_display::display::{Monitor, MAX_HEADS};
use nonos_intel_display::engine::access::{Clock, Mmio};
use nonos_intel_display::engine::gen::Gen;
use nonos_intel_display::engine::scanout::leftover;
use nonos_intel_display::engine::takeover::Output;

use super::take_head::run;

pub struct Plan {
    /// Each lit output, its monitor, and the entries its window needs.
    pub outs: [Option<(Output, Monitor, u32)>; MAX_HEADS],
    /// The lowest entry this driver's windows may take.
    pub ours: u32,
    /// The top of firmware's surfaces, leftovers aside.
    pub fw_top: u32,
}

/// The entries past the end of `o`'s lit surface.
pub fn fw_end(o: &Output) -> u32 {
    (o.plane.live / 4096).saturating_add(run(o.plane.stride / 4, o.plane.height))
}

pub fn plan(
    m: &mut dyn Mmio,
    clock: &dyn Clock,
    gen: &Gen,
    lit: [Option<Output>; MAX_HEADS],
    total: u32,
) -> Plan {
    let mut outs = [None; MAX_HEADS];
    let mut need_all = 0u32;
    for (slot, out) in outs.iter_mut().zip(lit) {
        let Some(out) = out else { continue };
        let monitor = Monitor::of(m, clock, gen, out.transcoder);
        let p = monitor.panel();
        let big = p.modes[..p.len]
            .iter()
            .fold((out.width, out.height), |a, m| (a.0.max(m.width), a.1.max(m.height)));
        // Two surfaces of the largest mode, and 64 entries for the cursor.
        let need = 2 * run(big.0, big.1) + 64;
        need_all = need_all.saturating_add(need);
        *slot = Some((out, monitor, need));
    }
    let ours = (total / 64 * 64).saturating_sub(need_all).min(total / 2);
    let firmware =
        outs.iter().flatten().filter(|o| !leftover(&o.0.plane, ours)).map(|o| fw_end(&o.0));
    Plan { outs, ours, fw_top: firmware.max().unwrap_or(0) }
}
