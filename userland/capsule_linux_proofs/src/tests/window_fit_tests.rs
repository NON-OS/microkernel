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

//! A guest window made full screen and back (wayland/fit.rs and
//! wayland/configure.rs). A guest window had one size for good: nothing told
//! the guest another, its set_fullscreen went unserved, and the window
//! manager never heard it was meant to fill the display.

use crate::configure::configure;
use crate::fit::{centred, origin, Configure, Fit, Mode, STATE_FULLSCREEN, STATE_MAXIMIZED};

/// The green button's rect on 1920x1080: below the 58 row menubar the shell
/// draws there, down to the bottom edge (app_skeleton chrome::full_screen).
const FILL: (u32, u32, u32, u32) = (0, 58, 1920, 1022);
/// Qwen's window where the window manager placed it.
const QWEN: (u32, u32, u32, u32) = (520, 284, 880, 560);

fn words(bytes: &[u8]) -> Vec<u32> {
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

#[test]
fn restoring_gives_back_the_rect_the_window_had() {
    let mut fit = Fit::default();
    let s = fit.ask(Mode::FullScreen, Some(QWEN), FILL, 9).expect("full screen").serial;
    fit.ack(s);
    fit.shown();
    let back = fit.ask(Mode::Normal, Some(FILL), FILL, 10).expect("restore");
    assert_eq!(back.rect, Some(QWEN));
    assert!(back.mode.states().is_empty());
    let mut out = Vec::new();
    configure(30, 29, &back, &mut out);
    assert_eq!(&words(&out)[2..5], &[880, 560, 0]);
    fit.ack(10);
    assert_eq!(origin(fit.due(), Some((0, 58)), 880, 560), (520, 284));
    assert!(!fit.due().expect("due").mode.fills(), "the dock would stay hidden");
}

#[test]
fn maximised_takes_the_same_rect_and_says_maximized() {
    let mut fit = Fit::default();
    let told = fit.ask(Mode::Maximized, Some(QWEN), FILL, 4).expect("a configure");
    assert_eq!(told.rect, Some(FILL));
    assert_eq!(told.mode.states(), &[STATE_MAXIMIZED]);
    assert!(told.mode.fills());
}

#[test]
fn asking_again_on_the_way_sends_nothing_and_a_replaced_ack_is_ignored() {
    let mut fit = Fit::default();
    assert!(fit.ask(Mode::FullScreen, Some(QWEN), FILL, 4).is_some());
    assert!(fit.ask(Mode::FullScreen, Some(QWEN), FILL, 5).is_none());
    // F11 again before the guest answered: back, to the rect it had.
    let back = fit.ask(Mode::Normal, Some(QWEN), FILL, 6).expect("back");
    assert_eq!(back.rect, Some(QWEN));
    fit.ack(4);
    assert_eq!(fit.due(), None, "an ack of the replaced configure took it");
    fit.ack(6);
    assert_eq!(fit.due().map(|c| c.mode), Some(Mode::Normal));
}

#[test]
fn turning_back_to_full_screen_on_the_way_out_keeps_the_rect_the_window_had() {
    let mut fit = Fit::default();
    let s = fit.ask(Mode::FullScreen, Some(QWEN), FILL, 3).expect("full screen").serial;
    fit.ack(s);
    fit.shown();
    // F11 twice before the guest answered the first: still full screen.
    assert!(fit.ask(Mode::Normal, Some(FILL), FILL, 4).is_some());
    let again = fit.ask(Mode::FullScreen, Some(FILL), FILL, 5).expect("full screen again");
    fit.ack(again.serial);
    fit.shown();
    let back = fit.ask(Mode::Normal, Some(FILL), FILL, 6).expect("back");
    assert_eq!(back.rect, Some(QWEN), "the full-screen rect was kept as the window's own");
}

#[test]
fn a_window_full_screen_before_it_was_ever_shown_comes_back_at_its_own_size() {
    let mut fit = Fit::default();
    let s = fit.ask(Mode::FullScreen, None, FILL, 3).expect("full screen").serial;
    fit.ack(s);
    fit.shown();
    let back = fit.ask(Mode::Normal, Some(FILL), FILL, 4).expect("back");
    assert_eq!(back.size(), (0, 0), "the guest picks its size");
    fit.ack(4);
    assert_eq!(origin(fit.due(), Some((0, 58)), 880, 560), centred(880, 560));
}
