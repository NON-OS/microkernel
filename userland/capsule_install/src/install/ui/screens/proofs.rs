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

//! Every STARK proof this boot checked, before a disk is chosen: one row
//! each, its state word, what the word rests on, and the short hex of what
//! was proven, so a person compares it against the boot log by eye. A word
//! other than VERIFIED is not a pass, and a question the kernel did not
//! answer reads UNKNOWN, never VERIFIED.

use nonos_app_skeleton::PaintBuffer;

use super::proofs_row::row;
use crate::install::proofs::rows;
use crate::install::state::State;
use crate::install::ui::frame::Body;
use crate::install::ui::theme;
use crate::install::ui::wrap::{paragraph, Ink};

const INTRO: &str = "The bootloader proved the kernel before it ran it; the kernel proved the \
bootloader that started it, and proves every capsule before it starts one. This is what each \
proof found on this boot.";

pub fn paint(state: &State, fb: &mut PaintBuffer, b: Body) {
    let m = &b.m;
    let mut r = paragraph(fb, b.x, b.y, b.w, INTRO, Ink::body(m, theme::MUTED)) + m.gap;
    for (i, proof) in rows(&state.boot).iter().enumerate() {
        r = row(fb, m, b.x, r, b.w, i + 1, proof);
    }
    fb.fill_rect(b.x, r, b.w, 1, theme::RULE);
}
