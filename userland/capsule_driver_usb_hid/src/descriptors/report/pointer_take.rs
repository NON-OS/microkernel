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

//! A pointer collection's Input item, field by field.

use super::input::Input;
use super::layout::NONE;
use super::pointer::Pointer;

const X: u32 = 0x0001_0030;
const Y: u32 = 0x0001_0031;
const WHEEL: u32 = 0x0001_0038;
const BUTTON_1: u32 = 0x0009_0001;
const BUTTON_5: u32 = 0x0009_0005;
const IN_RANGE: u32 = 0x000D_0032;
const TIP: u32 = 0x000D_0042;

pub fn take(part: &mut Option<Pointer>, inp: &Input) {
    if !inp.variable {
        return;
    }
    for i in 0..inp.g.count {
        let usage = inp.l.usage(i);
        if !wanted(usage, inp) {
            continue;
        }
        let Some(p) = mine(part, inp) else { return };
        match usage {
            X => _ = p.x.get_or_insert(inp.field(i)),
            Y => _ = p.y.get_or_insert(inp.field(i)),
            WHEEL => _ = p.wheel.get_or_insert(inp.field(i)),
            TIP => first(&mut p.tip, inp.field_at(i)),
            IN_RANGE => first(&mut p.range, inp.field_at(i)),
            _ => first(&mut p.buttons[(usage - BUTTON_1) as usize], inp.field_at(i)),
        }
    }
}

fn wanted(usage: u32, inp: &Input) -> bool {
    let wide = (1..=32).contains(&inp.g.size);
    match usage {
        X | Y => wide,
        WHEEL => wide && inp.relative,
        BUTTON_1..=BUTTON_5 | IN_RANGE | TIP => inp.g.size == 1,
        _ => false,
    }
}

/// Each field keeps the first bit that names it: a multi-touch report
/// repeats X and Y for every contact, and the first is the primary one.
fn first(bit: &mut u16, at: u16) {
    if *bit == NONE {
        *bit = at;
    }
}

fn mine<'a>(part: &'a mut Option<Pointer>, inp: &Input) -> Option<&'a mut Pointer> {
    let p = part.get_or_insert(Pointer::new(inp.id, inp.app));
    (p.id == inp.id && p.app == inp.app).then_some(p)
}
