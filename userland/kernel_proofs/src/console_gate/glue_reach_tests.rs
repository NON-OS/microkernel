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

//! The framebuffer's address reached by any accessor, not one spelling of
//! it: only the five gated draws and the boot log's pixel helper read a
//! field of `.base_va`, and only those five draws ask the gate.

use super::glue_sites_tests::BLIT_RS;
use super::text_items::naming;

const HEART: &str = "interrupts/timer/heartbeat/";

#[test]
fn only_the_gated_draws_and_the_pixel_helper_touch_the_address() {
    let bar = format!("{HEART}draw_bar.rs");
    let tick = format!("{HEART}on_tick.rs");
    let blip = format!("{HEART}paint_blip.rs");
    let (bar, tick, blip) = (bar.as_str(), tick.as_str(), blip.as_str());
    let through = [bar, tick, blip, "sys/boot_log/draw.rs", BLIT_RS];
    assert_eq!(naming(".base_va."), through);
    let gate = "kernel_core/init/framebuffer/drawing.rs";
    let asking = [bar, tick, blip, gate, "sys/boot_log/render.rs", BLIT_RS];
    assert_eq!(naming("drawable("), asking);
}
