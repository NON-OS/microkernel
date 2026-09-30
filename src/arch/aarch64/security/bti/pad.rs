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

pub const BTI_C: u32 = 0xD503245F;
pub const BTI_J: u32 = 0xD503249F;
pub const BTI_JC: u32 = 0xD50324DF;
pub const PACIASP: u32 = 0xD503233F;
pub const PACIBSP: u32 = 0xD503237F;

/* Whether an indirect branch may land on this instruction in a guarded page.
BTI c, j and jc are landing pads, and PACIASP and PACIBSP act as BTI c. A NOP
or a bare BTI accepts no branch type, so on a core with FEAT_BTI a branch to
either raises a Branch Target exception. */
pub const fn is_bti_landing_pad(instruction: u32) -> bool {
    matches!(instruction, BTI_C | BTI_J | BTI_JC | PACIASP | PACIBSP)
}
