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
use crate::install::ui::widgets::{card, kv};
use crate::install::ui::wrap::{paragraph, Ink};
use crate::install::ui::{text, theme};

const INTRO: &str = "Installs the system you are running onto a disk in this computer, with \
its store and an encrypted data volume. The disk you choose is erased. Nothing else is touched.";

pub fn paint(state: &State, fb: &mut PaintBuffer, b: Body) {
    let m = &b.m;
    let mut y = paragraph(fb, b.x, b.y, b.w, INTRO, Ink::body(m, theme::FOREGROUND));
    y += m.gap;
    let caption = "what will be written, and this machine";
    let inner = card(fb, m, b.x, y, b.w, m.card_h(6), caption);
    let x = b.x + m.inset;
    let w = b.w - 2 * m.inset;
    match &state.image {
        Some(image) => {
            let loader = bytes(image.loader.len() as u64);
            let mut r = kv(fb, m, x, inner, w, "bootloader", &loader, false);
            r = kv(fb, m, x, r, w, "kernel image", &bytes(image.kernel.len() as u64), false);
            let measured = hex_prefix(&state.boot.kernel_blake3);
            r = kv(fb, m, x, r, w, "kernel measurement", &measured, true);
            r = kv(fb, m, x, r, w, "boot verdict", state.boot.verdict(), false);
            let sb = if state.boot.secure_boot { "on" } else { "off" };
            r = kv(fb, m, x, r, w, "firmware secure boot", sb, false);
            kv(fb, m, x, r, w, "TPM", state.boot.tpm.text(), false);
        }
        None => {
            let why = state.notice.as_deref().unwrap_or("the image is not available");
            text::line(fb, x, inner, why, theme::DANGER, m.body_px);
            let after = "Nothing can be installed from this boot.";
            text::line(fb, x, inner + m.line_h, after, theme::MUTED, m.body_px);
        }
    }
}
