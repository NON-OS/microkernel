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
//! A killed driver's leftovers laid out by hand on the model (G-78): what
//! the death sweeps (`death_tests`) leave by running the driver itself.

use crate::engine::access::Mmio;
use crate::engine::gen::GEN9;
use crate::engine::ggtt::map;
use crate::engine::ggtt_pte::Pte;
use crate::engine::pipe::Pipe;
use crate::engine::{cursor, plane::program};
use crate::model::Gen9;

use super::kit::{pages, TOTAL};

pub const SECRET: u32 = 0xFF5E_C437;
/// The killed driver's frames: two surfaces, its cursor image, and a stray.
pub const DEAD: [u64; 4] = [0x3000_0000, 0x3400_0000, 0x3800_0000, 0x3900_0000];

/// Pipe `p` as a killed driver leaves it: its `w` x `h` surfaces mapped
/// below entry `top`, the plane latched on the first and armed on the
/// second (a flip cut off), its cursor on, and a stray entry low down.
pub fn leftovers(m: &mut Gen9, p: Pipe, top: u32, (w, h): (u32, u32)) {
    let n = (w * 4 * h).div_ceil(4096);
    let at = [top - n.next_multiple_of(64), top - 2 * n.next_multiple_of(64)];
    for i in 0..2 {
        m.fill(DEAD[i], n as u64, SECRET);
        map(m, Pte::Wide, at[i], &pages(DEAD[i], n), TOTAL).unwrap();
    }
    let ctl = m.read(crate::engine::plane_regs::plane_ctl(p, 1));
    program(m, GEN9.plane, p, at[0] * 4096, w * 4, (w, h), ctl).unwrap();
    m.fill(DEAD[2], 4, SECRET);
    map(m, Pte::Wide, at[1] - 64, &pages(DEAD[2], 4), TOTAL).unwrap();
    cursor::set(m, p, Some((at[1] - 64) * 4096), 10, 10).unwrap();
    m.vblank();
    m.write(crate::engine::plane_regs::plane_surf(p, 1), at[1] * 4096);
    map(m, Pte::Wide, 4096, &[DEAD[3]], TOTAL).unwrap();
}
