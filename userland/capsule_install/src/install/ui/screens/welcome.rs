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

//! What this does, and what it is about to write: the image this machine
//! booted, with the verdict the bootloader recorded about it. A person
//! reads the verdict before they pick a disk.

use nonos_app_skeleton::PaintBuffer;

use crate::install::format::{bytes, hex_prefix};
use crate::install::state::State;
use crate::install::ui::frame::Body;
use crate::install::ui::metrics::{BODY_PX, LINE_H};
use crate::install::ui::widgets::{card, kv};
use crate::install::ui::wrap::paragraph;
use crate::install::ui::{text, theme};

const INTRO: &str = "NØNOS runs from memory and keeps nothing on disk. Installing puts the \
image you are running now onto a drive in this computer so it can boot without the stick. \
The drive you choose is erased. Nothing else on the computer is touched.";

pub fn paint(state: &State, fb: &mut PaintBuffer, b: Body) {
    let mut y = paragraph(fb, b.x, b.y, b.w, INTRO, theme::FOREGROUND, BODY_PX, LINE_H);
    y += 12;
    let inner = card(fb, b.x, y, b.w, 5 * LINE_H + 40, "what will be written");
    let x = b.x + 16;
    let w = b.w - 32;
    match &state.image {
        Some(image) => {
            let mut r = kv(fb, x, inner, w, "bootloader", &bytes(image.loader.len() as u64), false);
            r = kv(fb, x, r, w, "kernel image", &bytes(image.kernel.len() as u64), false);
            r = kv(fb, x, r, w, "kernel measurement", &hex_prefix(&state.boot.kernel_blake3), true);
            r = kv(fb, x, r, w, "boot verdict", state.boot.verdict(), false);
            let sb = if state.boot.secure_boot { "on" } else { "off" };
            kv(fb, x, r, w, "firmware secure boot", sb, false);
        }
        None => {
            let why = state.notice.as_deref().unwrap_or("the image is not available");
            text::line(fb, x, inner, why, theme::DANGER, BODY_PX);
            text::line(
                fb,
                x,
                inner + LINE_H,
                "Nothing can be installed from this boot.",
                theme::MUTED,
                BODY_PX,
            );
        }
    }
}
