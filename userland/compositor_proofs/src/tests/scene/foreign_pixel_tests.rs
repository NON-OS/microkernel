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

//! Real pixels, composed by the compositor's own `compose`, from surfaces in
//! memory the way the kernel hands them over. A surface whose owner exits
//! goes back to the kernel, which gives its frames to other processes: an
//! image viewer's decoded picture, a wallpaper. Here that memory is filled
//! with FOREIGN the moment the owner exits, and no composed pixel may ever
//! be FOREIGN again. The pointer moves across the screen in a staircase, as
//! on ek's screen, so every rectangle it damages is composed afresh.

use super::foreign_desk::*;
use super::foreign_kernel::*;

#[test]
fn a_window_whose_owner_exited_never_shows_reused_memory() {
    let mut d = Desk::new();
    d.kernel.register(20, 50, 36, 50, WINDOW);
    d.submit(2, 20, (20, 12, 50, 36), 2);
    d.frame();
    d.assert_current("before the exit");
    // The guest closes and its process ends with no scene remove.
    d.kernel.exit(20);
    d.staircase("after the exit");
    assert!(d.scene.layers().all(|l| l.owner_pid != 2), "the dead window's layer went");
    d.assert_current("after the exit");
}

#[test]
fn a_handle_number_given_to_another_owner_shows_only_the_new_surface() {
    let mut d = Desk::new();
    d.kernel.register(20, 50, 36, 50, WINDOW);
    d.submit(2, 20, (20, 12, 50, 36), 2);
    d.frame();
    d.kernel.exit(20);
    d.staircase("after the exit");
    // Another process registers a surface and the kernel hands it the same
    // handle number; the compositor's old mapping of it must not be used.
    d.kernel.reissue(20, 30, 20, OTHER);
    d.submit(4, 20, (50, 30, 30, 20), 2);
    d.frame();
    d.staircase("after the reuse");
    d.assert_current("after the reuse");
    assert_eq!(d.canvas[(40 * W + 60) as usize], OTHER, "the new window is drawn");
}

#[test]
fn a_layer_larger_than_its_surface_reads_nothing_past_it() {
    let mut d = Desk::new();
    // A resize in flight: the layer is submitted at the new size while the
    // surface is still the old, smaller one, with row padding in its stride.
    d.kernel.register(30, 20, 12, 28, WINDOW);
    d.submit(3, 30, (10, 10, 60, 40), 2);
    d.frame();
    d.assert_clean("a short surface");
    d.staircase("a short surface");
    d.assert_current("a short surface");
    assert_eq!(d.canvas[(15 * W + 15) as usize], WINDOW);
    assert_eq!(d.canvas[(45 * W + 60) as usize], WALL, "past the surface the wallpaper shows");
}
