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
//! A dark plane on a running transcoder, taken only when it is this
//! driver's own leftover: a mode set killed after it turned the
//! plane off leaves the pipe running with nothing shown, which G-12 reads
//! as no output. It is taken when what bring-up reads says this driver
//! set it: the pipe's picture is its timing's active size, as a mode set
//! leaves it; the plane holds the linear 32-bit layout this
//! driver programs; and the last surface it showed lies in the upper half
//! of the GGTT, where this driver's windows go and firmware's picture never
//! does. That surface's entries are not asked for: the mode set takes them
//! out as soon as the plane is off. Anything else stays dark.

use super::access::{field, Mmio};
use super::gen::Gen;
use super::pipe::Pipe;
use super::plane_read::PlaneState;
use super::regs;
use super::scanout::surface_run;
use super::takeover::{feeding, read_output, running, Output};
use super::timing;

/// Every output to take, pipe A first: lit ones, and this driver's dark
/// leftovers in a GGTT of `total` entries.
pub fn read_outputs_kept(m: &mut dyn Mmio, g: &Gen, total: u32) -> [Option<Output>; 4] {
    Pipe::ALL.map(|p| read_output(m, g, p).or_else(|| dark_leftover(m, g, p, total)))
}

pub fn dark_leftover(m: &mut dyn Mmio, g: &Gen, p: Pipe, total: u32) -> Option<Output> {
    (p.index() < g.pipes).then_some(())?;
    let transcoder = feeding(m, g, p);
    running(m, transcoder).then_some(())?;
    let timing = timing::read(m, transcoder)?;
    let src = m.read(regs::pipe_src(p));
    let (width, height) = (field(src, 28, 16) + 1, field(src, 11, 0) + 1);
    let set = (width, height) == (timing.h_active, timing.v_active);
    let plane = PlaneState::read(m, g.plane, p);
    let ours = high(&plane, total) && plane.flippable() && !plane.enabled();
    (set && ours).then_some(())?;
    Some(Output { pipe: p, transcoder, timing, width, height, plane, ggtt: g.ggtt })
}

/// Whether `plane`'s last surface lies whole in the GGTT's upper half.
fn high(plane: &PlaneState, total: u32) -> bool {
    let (first, pages) = surface_run(plane);
    pages > 0 && first >= total / 2 && first as u64 + pages as u64 <= total as u64
}
