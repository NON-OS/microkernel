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

//! One whole drawing of the splash into a buffer: the wash over every
//! pixel first, so nothing that was on the canvas before (the firmware's
//! screen above all) shows through, then NONOS's title, the attestation
//! panel, the warning when device DMA is unconfined, and the status line.
//! Pure; held in boot_splash_proofs.

use nonos_toolkit::font::render::draw_text;

use crate::attest_lines::{lines, Chain};
use crate::chrome::panel;
use crate::layout::Area;
use crate::layout_fit::Layout;

/// What one drawing shows besides its size.
#[derive(Clone, Copy)]
pub struct Canvas {
    pub fit: Layout,
    pub chain: Option<Chain>,
    pub dma_unconfined: bool,
}

pub fn draw(buf: &mut [u32], spx: usize, w: u32, h: u32, c: &Canvas) {
    crate::vignette::fill(buf, spx, w, h);
    crate::title::draw(buf, spx, w, h);
    attest_panel(buf, spx, (w, h), c.fit.panel, c.chain);
    if c.dma_unconfined {
        crate::warning::draw(buf, spx, w, h, c.fit.warn);
    }
    crate::status::status(buf, spx, w, h, c.fit.status, 0);
}

/// Each line says what this boot recorded (attest_lines.rs), 20 px apart.
fn attest_panel(buf: &mut [u32], spx: usize, (w, h): (u32, u32), at: Area, chain: Option<Chain>) {
    let (px, py, _, _) = at;
    panel(buf, spx, (w, h), at, b"boot-chain attestation");
    for (i, (text, colour)) in lines(chain).into_iter().enumerate() {
        draw_text(buf, spx, w, h, px + 14, py + 32 + 20 * i as u32, text, colour);
    }
}
