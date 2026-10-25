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

use nonos_toolkit::font::render::draw_text;

use crate::attest_lines::{lines, Chain};
use crate::chrome::panel;
use crate::layout::Area;
use crate::look::Look;

/// The firmware's logo, when it left one and the layout keeps it, stands
/// where the title would.
pub(crate) fn splash(base: u64, w: u32, h: u32, stride: u32, look: &Look) {
    let spx = stride as usize / 4;
    let buf = unsafe { core::slice::from_raw_parts_mut(base as *mut u32, spx * h as usize) };
    crate::vignette::fill(buf, spx, w, h);
    match look.logo.as_ref().filter(|_| look.fit.logo) {
        Some(l) => l.draw(buf, spx, w, h),
        None => crate::title::draw(buf, spx, w, h),
    }
    attest_panel(buf, spx, (w, h), look.fit.panel, look.chain);
    if look.dma_unconfined {
        crate::warning::draw(buf, spx, w, h, look.fit.warn);
    }
    crate::status::status(buf, spx, w, h, look.fit.status, 0);
}

/// Each line says what this boot recorded (attest_lines.rs), 20 px apart.
fn attest_panel(buf: &mut [u32], spx: usize, (w, h): (u32, u32), at: Area, chain: Option<Chain>) {
    let (px, py, pw, ph) = at;
    panel(buf, spx, w, h, px, py, pw, ph, b"boot-chain attestation");
    for (i, (text, colour)) in lines(chain).into_iter().enumerate() {
        draw_text(buf, spx, w, h, px + 14, py + 32 + 20 * i as u32, text, colour);
    }
}
