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

//! Where a boot that cannot go on stops. It says the step on serial, as
//! `[FATAL] <step>: <detail>`, and on the panel as a notice band, because
//! most laptops have no serial port and a photo of the screen is all the
//! owner gets back. The band is drawn through whichever framebuffer exists
//! by then (sys::boot_log screen.rs), so this works from the first stage on.

use crate::sys::{boot_log, serial};

const TITLE: &[u8] = b"NONOS BOOT STOPPED";
const HINT: &[u8] = b"Photograph this screen. The serial log has the rest.";

pub fn stop(step: &str, detail: &str) -> ! {
    serial::print(b"[FATAL] ");
    serial::print_str(step);
    if !detail.is_empty() {
        serial::print(b": ");
        serial::print_str(detail);
    }
    serial::println(b"");
    if detail.is_empty() {
        boot_log::show_notice(TITLE, &[step.as_bytes(), HINT]);
    } else {
        boot_log::show_notice(TITLE, &[step.as_bytes(), detail.as_bytes(), HINT]);
    }
    crate::arch::halt_loop()
}
