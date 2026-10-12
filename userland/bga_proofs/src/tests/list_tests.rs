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

//! bga in the kernel's display list: the compositor finds it there, and
//! the kernel holds it to the compositor because it is there.

use crate::displays::{is_display, DISPLAYS};

#[test]
fn the_service_bga_serves_is_a_display() {
    assert!(is_display("driver.bga0"));
}

#[test]
fn bga_comes_after_the_paravirtual_displays() {
    /*
     * virtio-gpu and the hypervisors' own displays carry hints and a
     * cursor; DISPI has neither, so it is the emulated fallback.
     */
    let at = |n| DISPLAYS.iter().position(|d| *d == n);
    assert!(at("driver.virtio_gpu0") < at("driver.bga0"));
}
