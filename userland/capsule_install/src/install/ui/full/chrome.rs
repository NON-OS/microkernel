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

/*
 * The left panel, as on the bootloader's screens: the NØNOS emblem, what
 * this is, and the steps as numbered mono rows, the current one lit. Setup
 * shows as done only when it kept answers for the install to carry. The
 * layout centres the block top to bottom and picks its row height.
 */

use alloc::format;

use nonos_app_skeleton::PaintBuffer;

use crate::install::full::setup_kept;
use crate::install::state::Screen;

use super::layout::FullLayout;
use super::steps::{centred, row};
use nonos_brand::emblem;
use nonos_brand::palette::{CYAN, RULE, TEXT, TEXT_3};

const STEPS: [&str; 5] =
    ["THIS COMPUTER", "CHOOSE THE DISK", "ERASE PLAN", "WRITE, READ BACK", "INSTALLED"];

pub(super) fn panel(fb: &mut PaintBuffer, l: &FullLayout, screen: Screen) {
    let (h, pw) = (fb.height, l.panel_w);
    fb.fill_rect(pw, 0, l.scale.px(1).max(1), h, RULE);
    emblem(fb, l.emblem_x, l.emblem_y, l.emblem_w, CYAN, true);
    centred(fb, l, l.caption_y, "INSTALL N\u{d8}NOS", TEXT);
    centred(fb, l, l.release_y, &nonos_brand::release(), TEXT_3);
    let now = screen.step() as usize;
    let (x, mut y) = (l.steps_x, l.steps_y);
    let setup = if setup_kept() { "FIRST-BOOT SETUP" } else { "SETUP: NO ANSWERS KEPT" };
    row(fb, l, x, y, "00", setup, if setup_kept() { CYAN } else { TEXT_3 }, false);
    for (i, name) in STEPS.iter().enumerate() {
        y += l.step_h;
        let colour = if i + 1 < now {
            CYAN
        } else if i + 1 == now {
            TEXT
        } else {
            TEXT_3
        };
        row(fb, l, x, y, &format!("{:02}", i + 1), name, colour, i + 1 == now);
    }
}
