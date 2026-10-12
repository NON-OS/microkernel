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

//! What the monitor's EDID tells this driver: its preferred mode, its size
//! in millimetres, and the modes it lists that DISPI can show, each at
//! most once (nonos_edid reads the blob; nothing is allocated).

use nonos_edid::Edid;
use nonos_gfx_wire::body::mode::{FROM_EDID, MAX_MODES, PREFERRED};
use nonos_gfx_wire::Mode;

#[derive(Clone, Copy, Debug, Default)]
pub struct Panel {
    pub preferred: Option<(u32, u32)>,
    pub physical_mm: u32,
    pub modes: [Mode; MAX_MODES],
    pub len: usize,
}

impl Panel {
    /// From the blob's bytes, keeping modes `fits` accepts; an empty panel
    /// when there is no blob or it does not parse.
    pub fn read(blob: &[u8], fits: impl Fn(u32, u32) -> bool) -> Panel {
        let mut p = Panel::default();
        let Ok(edid) = Edid::parse(blob) else { return p };
        p.physical_mm = edid.physical_mm();
        p.preferred = edid.preferred().map(|d| (d.width(), d.height())).filter(|m| fits(m.0, m.1));
        edid.for_each_timing(&mut |t| {
            let (w, h) = (t.width, t.height);
            let seen = p.modes[..p.len].iter().any(|m| (m.width, m.height) == (w, h));
            if !t.interlaced && fits(w, h) && !seen && p.len < MAX_MODES {
                let pref = if p.preferred == Some((w, h)) { PREFERRED } else { 0 };
                p.modes[p.len] = Mode { width: w, height: h, flags: FROM_EDID | pref };
                p.len += 1;
            }
        });
        p
    }
}
