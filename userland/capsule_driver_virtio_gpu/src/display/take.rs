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
//! Bring-up's takeover: every scanout the host enabled, up to the
//! wire's outputs, gets a surface at the size the host asked for, zeroed
//! and sent to the host whole, then set on its scanout and flushed. A
//! scanout whose surface cannot be made is left as the host has it.

use alloc::vec::Vec;

use nonos_gfx_serve::{Panel, MAX_OUTPUTS};
use nonos_gfx_wire::Rect;

use super::gpu::{Gpu, Head};
use super::plug::Plug;
use super::show::area;
use crate::constants::VG_MAX_SCANOUTS;
use crate::device::cmd::set_scanout;
use crate::device::ControlQueue;
use crate::state::{FenceCounter, Health, ScanoutTable};

pub fn take(
    (q, fences): (ControlQueue, FenceCounter),
    device: (u64, u64),
    table: &ScanoutTable,
    panel: Panel,
) -> Result<Gpu, &'static str> {
    let heads = [const { None }; MAX_OUTPUTS];
    let (limbo, health, outputs) = (Vec::new(), Health::Live, 0);
    let mut g =
        Gpu { q, fences, device, heads, outputs, panel, generation: 1, next_id: 1, limbo, health };
    for id in 0..VG_MAX_SCANOUTS as u32 {
        let Some(s) = table.get(id).filter(|s| s.enabled) else { continue };
        if g.outputs as usize == MAX_OUTPUTS {
            break;
        }
        let Ok(shown) = g.make(s.width, s.height) else { continue };
        let whole = [Rect { x: 0, y: 0, width: s.width, height: s.height }];
        // The kernel zeroed the pages: the host's copy is made zero too.
        let lit = g.send(&shown, &whole).is_ok()
            && set_scanout(&g.q, g.fences.issue(), id, shown.id, area(&whole[0])).is_ok();
        if !lit {
            g.give_back(shown);
            continue;
        }
        let _ = g.flush_rects(&shown, &whole);
        g.heads[g.outputs as usize] =
            Some(Head { scanout: id, shown, hint: (s.width, s.height), plug: Plug::In });
        g.outputs += 1;
    }
    if g.outputs == 0 {
        return Err("virtio-gpu: no scanout could be taken");
    }
    Ok(g)
}
