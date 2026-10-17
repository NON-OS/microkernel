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
//! MODES: the host's wanted size first (the device's own mode, section
//! 12's first choice), then the modes output 0's EDID lists, then the size
//! shown now; each once, flagged by where it came from.

use nonos_gfx_wire::body::mode::{CURRENT, FROM_DEVICE, PREFERRED};
use nonos_gfx_wire::Mode;

use super::gpu::Gpu;

pub fn modes(g: &Gpu, output: u16, out: &mut [Mode]) -> usize {
    let Some(h) = g.head(output) else { return 0 };
    let shown = (h.shown.width, h.shown.height);
    let host = (h.hint.0 > 0 && h.hint.1 > 0).then_some(Mode {
        width: h.hint.0,
        height: h.hint.1,
        flags: FROM_DEVICE | PREFERRED,
    });
    // One preferred mode: the host's when it names one, else EDID's.
    let keep = if host.is_some() { !PREFERRED } else { u32::MAX };
    let edid = if output == 0 { &g.panel.modes[..g.panel.len] } else { &[][..] };
    let edid = edid.iter().map(|m| Mode { flags: m.flags & keep, ..*m });
    let now = Mode { width: shown.0, height: shown.1, flags: 0 };
    let mut len = 0;
    for m in host.into_iter().chain(edid).chain(core::iter::once(now)) {
        let flag = if (m.width, m.height) == shown { CURRENT } else { 0 };
        let same = |o: &&mut Mode| (o.width, o.height) == (m.width, m.height);
        if let Some(o) = out[..len].iter_mut().find(same) {
            o.flags |= flag;
            continue;
        }
        let Some(slot) = out.get_mut(len) else { break };
        *slot = Mode { flags: m.flags | flag, ..m };
        len += 1;
    }
    len
}
