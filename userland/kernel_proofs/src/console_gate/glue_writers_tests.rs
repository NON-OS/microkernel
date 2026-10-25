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

//! The console's other writers, found by walking src/. Only the gate's own
//! module, the stop and panic screens, the blit's errno and two readers of
//! the geometry name the framebuffer's state; only the five gated draws and
//! the boot log's pixel helper write through its address, and that helper
//! is reached from the gated boot log and from the screens.

use super::glue_sites_tests::BLIT_RS;
use super::text::code;
use super::text_items::{body, flat, naming};

#[test]
fn only_these_name_the_framebuffers_state_and_two_read_only_its_geometry() {
    let readers = [
        "hardware/broker/bootfb/info.rs",
        "kernel_core/init/framebuffer/drawing.rs",
        "kernel_core/init/framebuffer/mod.rs",
        "kernel_core/init/framebuffer/state.rs",
        "sys/boot_log/screen.rs",
        "syscall/dispatch/router/graphics_backend.rs",
        "syscall/dispatch/router/graphics_present/blit.rs",
    ];
    assert_eq!(naming("framebuffer_state"), readers);
    for rel in [readers[0], readers[5]] {
        let text = code(rel);
        assert!(!text.contains("base_va") && !text.contains("write_volatile"), "{rel} draws");
    }
}

#[test]
fn only_the_gated_draws_and_the_screens_helper_write_through_its_address() {
    let writers = [
        "interrupts/timer/heartbeat/draw_bar.rs",
        "interrupts/timer/heartbeat/on_tick.rs",
        "interrupts/timer/heartbeat/paint_blip.rs",
        "sys/boot_log/draw.rs",
        "syscall/dispatch/router/graphics_present/blit.rs",
    ];
    assert_eq!(naming(".base_va.as_u64()"), writers);
    let helper: Vec<String> =
        naming("super::draw::").into_iter().filter(|f| f.starts_with("sys/boot_log/")).collect();
    let screens = ["sys/boot_log/notice_screen.rs", "sys/boot_log/panic_screen.rs"];
    assert_eq!(helper, [screens[0], screens[1], "sys/boot_log/render.rs"]);
    assert_eq!(naming("screen::screen"), screens);
    let marker = ["kernel_core/init/framebuffer/init.rs", "kernel_core/init/framebuffer/marker.rs"];
    assert_eq!(naming("paint_mapped_marker("), marker, "the mapping's marker painted again");
}

#[test]
fn the_blit_reads_the_state_only_to_pick_its_errno() {
    let b = flat(body(&code(BLIT_RS), "fn blit("));
    let errno = "letheld=framebuffer::framebuffer_state().is_some();";
    let pick = "returnutil::errno(ifheld{EBUSY}else{ENOTSUP});";
    assert!(b.contains(&format!("{errno}{pick}")), "the blit reads the state for more");
    assert_eq!(b.matches("framebuffer_state").count(), 1);
}
