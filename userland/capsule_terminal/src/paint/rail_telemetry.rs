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
use nonos_app_skeleton::PaintBuffer;
use nonos_toolkit::widgets::Scrolled;

use super::rail_text::gap;
use super::{rail_disk, rail_net, rail_sys};
use crate::rail::Rail;

/// The telemetry sections, laid out top to bottom from content row `y` of a
/// column `vh` tall scrolled by `off`. Each step advances by the section's own
/// `height()`, the same figure the scroll offset is clamped against, and a
/// section scrolled clear of the rail is skipped rather than rasterized.
pub fn draw(fb: &mut PaintBuffer, off: u32, x: u32, y: u32, w: u32, rail: &Rail) {
    let vh = fb.height;
    let shown = |y: u32, h: u32| y < off + vh && y + h > off;
    let mut y = y;
    if shown(y, rail_sys::height()) {
        rail_sys::draw(fb, off, x, y, w, rail);
    }
    y += rail_sys::height() + gap();
    if shown(y, rail_net::height()) {
        rail_net::draw(&mut Scrolled::new(fb, off), x, y, w, &rail.sample.net);
    }
    y += rail_net::height() + gap();
    if shown(y, rail_disk::height()) {
        rail_disk::draw(&mut Scrolled::new(fb, off), x, y, w, &rail.sample.disk);
    }
}
