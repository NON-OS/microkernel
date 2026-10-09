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


//! The instruction templates and the fixed layout that says which kind of
//! template each sub-cycle draws from. Latencies and execution ports are the
//! reference's model of an Ivy Bridge core.

use super::Op;

pub const PORT_NONE: u8 = 0;
pub const PORT_P0: u8 = 1;
pub const PORT_P1: u8 = 2;
pub const PORT_P5: u8 = 4;
const PORT_P01: u8 = PORT_P0 | PORT_P1;
const PORT_P05: u8 = PORT_P0 | PORT_P5;
const PORT_P015: u8 = PORT_P0 | PORT_P1 | PORT_P5;

/// A branch's immediate is not drawn like the others: it gets a mask of
/// four distinct bits. This sentinel in `immediate_mask` selects that.
pub const BRANCH_MASK: u32 = 0x8000_0000;

pub struct Template {
    pub op: Op,
    pub latency: i32,
    pub uop1: u8,
    pub uop2: u8,
    pub immediate_mask: u32,
    pub group: Op,
    pub imm_can_be_0: bool,
    pub distinct_dst: bool,
    pub op_par_src: bool,
    pub has_src: bool,
    pub has_dst: bool,
}

const fn tpl(op: Op, latency: i32, uop1: u8, uop2: u8, immediate_mask: u32, group: Op, flags: [bool; 5]) -> Template {
    Template {
        op,
        latency,
        uop1,
        uop2,
        immediate_mask,
        group,
        imm_can_be_0: flags[0],
        distinct_dst: flags[1],
        op_par_src: flags[2],
        has_src: flags[3],
        has_dst: flags[4],
    }
}

// Flags, in order: imm_can_be_0, distinct_dst, op_par_src, has_src, has_dst.
pub static UMULH_R: Template = tpl(Op::UmulhR, 4, PORT_P1, PORT_P5, 0, Op::UmulhR, [false, false, false, true, true]);
pub static SMULH_R: Template = tpl(Op::SmulhR, 4, PORT_P1, PORT_P5, 0, Op::SmulhR, [false, false, false, true, true]);
pub static MUL_R: Template = tpl(Op::MulR, 3, PORT_P1, PORT_NONE, 0, Op::MulR, [false, true, true, true, true]);
// Subtraction shares its group with ADD_RS on purpose, so the generator
// never puts the two back to back on one register.
pub static SUB_R: Template = tpl(Op::SubR, 1, PORT_P015, PORT_NONE, 0, Op::AddRs, [false, true, true, true, true]);
pub static XOR_R: Template = tpl(Op::XorR, 1, PORT_P015, PORT_NONE, 0, Op::XorR, [false, true, true, true, true]);
pub static ADD_RS: Template = tpl(Op::AddRs, 1, PORT_P01, PORT_NONE, 3, Op::AddRs, [true, true, true, true, true]);
pub static ROR_C: Template = tpl(Op::RorC, 1, PORT_P05, PORT_NONE, 63, Op::RorC, [false, true, false, false, true]);
pub static ADD_C: Template = tpl(Op::AddC, 1, PORT_P015, PORT_NONE, u32::MAX, Op::AddC, [false, true, false, false, true]);
pub static XOR_C: Template = tpl(Op::XorC, 1, PORT_P015, PORT_NONE, u32::MAX, Op::XorC, [false, true, false, false, true]);
pub static TARGET: Template = tpl(Op::Target, 1, PORT_P015, PORT_P015, 0, Op::Target, [false, true, false, false, false]);
pub static BRANCH: Template = tpl(Op::Branch, 1, PORT_P015, PORT_P015, BRANCH_MASK, Op::Branch, [false, true, false, false, false]);

/// What a slot in the layout may hold. `mask0` picks among `templates` on a
/// first attempt and `mask1` on a retry; a zero `mask0` means the slot has
/// one template and draws nothing.
pub struct Item {
    pub templates: &'static [&'static Template],
    pub mask0: u8,
    pub mask1: u8,
    pub duplicates: bool,
}

static ANY: [&Template; 8] = [&ROR_C, &XOR_C, &ADD_C, &ADD_C, &SUB_R, &XOR_R, &XOR_C, &ADD_RS];
static WIDE_MUL: [&Template; 2] = [&SMULH_R, &UMULH_R];
static ONE_MUL: [&Template; 1] = [&MUL_R];
static ONE_TARGET: [&Template; 1] = [&TARGET];
static ONE_BRANCH: [&Template; 1] = [&BRANCH];

static ITEM_MUL: Item = Item { templates: &ONE_MUL, mask0: 0, mask1: 0, duplicates: true };
static ITEM_TARGET: Item = Item { templates: &ONE_TARGET, mask0: 0, mask1: 0, duplicates: true };
static ITEM_BRANCH: Item = Item { templates: &ONE_BRANCH, mask0: 0, mask1: 0, duplicates: true };
static ITEM_WIDE_MUL: Item = Item { templates: &WIDE_MUL, mask0: 1, mask1: 1, duplicates: true };
// On a retry only the first four, which need no source register.
static ITEM_ANY: Item = Item { templates: &ANY, mask0: 7, mask1: 3, duplicates: false };

/// The 36-slot pattern the generator walks, one slot per sub-cycle.
pub static LAYOUT: [&Item; 36] = [
    &ITEM_MUL, &ITEM_TARGET, &ITEM_ANY, &ITEM_MUL, &ITEM_ANY, &ITEM_ANY,
    &ITEM_MUL, &ITEM_ANY, &ITEM_ANY, &ITEM_MUL, &ITEM_ANY, &ITEM_ANY,
    &ITEM_WIDE_MUL, &ITEM_ANY, &ITEM_ANY, &ITEM_MUL, &ITEM_ANY, &ITEM_ANY,
    &ITEM_MUL, &ITEM_BRANCH, &ITEM_ANY, &ITEM_MUL, &ITEM_ANY, &ITEM_ANY,
    &ITEM_WIDE_MUL, &ITEM_ANY, &ITEM_ANY, &ITEM_MUL, &ITEM_ANY, &ITEM_ANY,
    &ITEM_MUL, &ITEM_ANY, &ITEM_ANY, &ITEM_MUL, &ITEM_ANY, &ITEM_ANY,
];
