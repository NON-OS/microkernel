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
//! A takeover settled (G-78). A restarted driver may meet what its killed
//! self left: a cursor on, a plane it was flipping or cannot take, and GGTT
//! entries naming its DMA frames (kept for good by the kernel). With every
//! output that can be on this driver's surfaces, each lit cursor is hidden,
//! each untaken plane on a leftover turned off, a blank lets both latch,
//! and the GGTT is rewritten whole: every entry cleared but this driver's
//! surfaces and a picture firmware scans out on an output not taken.

use super::access::{Expiry, Mmio};
use super::cursor;
use super::ggtt::unmap;
use super::ggtt_pte::Pte;
use super::pipe::Pipe;
use super::plane_read::ENABLE;
use super::plane_regs as r;
use super::scanout::{leftover, surface_run};
use super::takeover::Output;
use super::vblank::wait_vblank;

/// A run of GGTT entries kept: (first entry, count).
pub type Run = (u32, u32);

/// `lit` as read, `taken` its pipes now on `keep`, `ours` the plan's.
pub fn settle(
    m: &mut dyn Mmio,
    (lit, taken): (&[Option<Output>], &[Pipe]),
    keep: &[Run],
    (t, ours, total): (Pte, u32, u32),
    until: &dyn Expiry,
) {
    // This driver's surfaces (two a head) and firmware's pictures left lit.
    let mut runs = [(total, 0); 2 * 4 + 4];
    let mut n = keep.len().min(8);
    runs[..n].copy_from_slice(&keep[..n]);
    for o in lit.iter().flatten() {
        let p = o.pipe;
        let _ = cursor::set(m, p, None, 0, 0);
        match (taken.contains(&p), leftover(&o.plane, ours)) {
            (true, _) => {}
            (false, true) => {
                let (ctl, surf) = (m.read(r::plane_ctl(p, 1)), m.read(r::plane_surf(p, 1)));
                m.write(r::plane_ctl(p, 1), ctl & !ENABLE);
                m.write(r::plane_surf(p, 1), surf);
            }
            (false, false) if n < runs.len() => (runs[n], n) = (surface_run(&o.plane), n + 1),
            (false, false) => {}
        }
    }
    // A pipe that never blanks scans nothing out; its entries go all the same.
    lit.iter().flatten().for_each(|o| _ = wait_vblank(m, o.pipe, until));
    runs.sort_unstable();
    let mut next = 0u32;
    for (first, count) in runs {
        if first > next {
            let _ = unmap(m, t, next, first.min(total) - next, total);
        }
        next = next.max(first.saturating_add(count));
    }
}
