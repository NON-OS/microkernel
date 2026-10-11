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

//! Full screen: the size the guest is told, and the fill once its buffer comes.

use crate::configure::configure;
use crate::fit::mode::STATE_FULLSCREEN;
use crate::fit::{origin, Configure, Fit, Mode};

/// The green button's rect on 1920x1080: below the 58 row menubar the shell
/// draws there, down to the bottom edge (app_skeleton chrome::full_screen).
pub(super) const FILL: (u32, u32, u32, u32) = (0, 58, 1920, 1022);

/// Qwen's window where the window manager placed it.
pub(super) const QWEN: (u32, u32, u32, u32) = (520, 284, 880, 560);

pub(super) fn words(bytes: &[u8]) -> Vec<u32> {
    bytes.chunks(4).map(|c| u32::from_le_bytes([c[0], c[1], c[2], c[3]])).collect()
}

#[test]
fn full_screen_tells_the_guest_the_display_below_the_menubar_and_the_state() {
    let mut fit = Fit::default();
    let told = fit.ask(Mode::FullScreen, Some(QWEN), FILL, 9).expect("a configure");
    assert_eq!(told, Configure { serial: 9, mode: Mode::FullScreen, rect: Some(FILL) });
    let mut out = Vec::new();
    configure(30, 29, &told, &mut out);
    // xdg_toplevel.configure: 1920, 1022, a states array of one word,
    // fullscreen; then xdg_surface.configure with the serial.
    let w = words(&out);
    assert_eq!(w[0], 30);
    // 24 bytes, opcode 0.
    assert_eq!(w[1], 24 << 16);
    assert_eq!(&w[2..6], &[1920, 1022, 4, STATE_FULLSCREEN]);
    assert_eq!(&w[6..9], &[29, 12 << 16, 9]);
    assert_eq!(w.len(), 9);
}

#[test]
fn the_window_fills_to_the_bottom_edge_once_the_buffer_for_it_comes() {
    let mut fit = Fit::default();
    let told = fit.ask(Mode::FullScreen, Some(QWEN), FILL, 9).expect("a configure");
    // Not acked: a buffer committed now is drawn for the old size, and is
    // shown where the window is.
    assert_eq!(fit.due(), None);
    assert_eq!(origin(fit.due(), Some((QWEN.0, QWEN.1)), 880, 560), (520, 284));
    fit.ack(told.serial);
    let due = fit.due().expect("due");
    let (x, y) = origin(Some(due), Some((QWEN.0, QWEN.1)), 1920, 1022);
    let (w, h) = due.size();
    assert_eq!((x, y, w, h), FILL);
    assert_eq!(y + h, 1080, "short of the bottom edge");
    assert!(due.mode.fills(), "the window manager would keep the dock");
    fit.shown();
    assert_eq!(fit.heading(), Mode::FullScreen);
    assert_eq!(fit.due(), None);
}
