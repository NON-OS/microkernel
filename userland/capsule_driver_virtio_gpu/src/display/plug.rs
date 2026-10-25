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
//! Whether an output's scanout is on, as the host's display events last
//! said (G-77). A host turns a scanout off when its window or monitor goes
//! and on again when it is back. Only a scanout taken at bring-up is an
//! output; one the host turns on later for the first time is not driven.

use super::gpu::Gpu;
use crate::device::cmd::get_display_info::DisplayInfo;
use crate::device::cmd::set_scanout;
use crate::device::cmd::transfer_to_host_2d::Rect as Area;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Plug {
    /// On, showing the head's surface.
    In,
    /// Off: the scanout let go of the surface, which is kept for when it
    /// is back, and nothing is sent for the output meanwhile.
    Out,
    /// On again: STATE says LOST, so the wire server sets the mode again
    /// and the surface is shown; until then nothing is sent for it.
    Back,
}

impl Gpu {
    /// Each head follows `info`: its wanted size taken when on, turned off
    /// or back when the host turned its scanout so. Whether any changed.
    pub(super) fn follow(&mut self, info: &DisplayInfo) -> bool {
        let Gpu { q, fences, heads, .. } = self;
        let mut changed = false;
        for h in heads.iter_mut().flatten() {
            let s = info.scanouts[h.scanout as usize];
            let on = s.enabled != 0 && s.width > 0 && s.height > 0;
            let was = (h.plug, h.hint);
            if on {
                h.hint = (s.width, s.height);
            }
            h.plug = match (h.plug, on) {
                (Plug::In | Plug::Back, false) => {
                    // Off the surface, so the host holds it nowhere shown;
                    // a host that refuses has turned the scanout off anyway.
                    let none = Area { x: 0, y: 0, width: 0, height: 0 };
                    let _ = set_scanout(q, fences.issue(), h.scanout, 0, none);
                    Plug::Out
                }
                (Plug::Out, true) => Plug::Back,
                (p, _) => p,
            };
            changed |= (h.plug, h.hint) != was;
        }
        changed
    }
}
