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

//! A window's state told again when only `activated` changed: its mode and
//! size as they are, or as they are on their way to, and the state words.

use super::full::{words, FILL, QWEN};
use crate::configure::{configure, STATE_ACTIVATED, TOPLEVEL_CONFIGURE_BOUNDS};
use crate::fit::mode::{STATE_FULLSCREEN, STATE_MAXIMIZED};
use crate::fit::{Configure, Fit, Mode};

#[test]
fn a_window_in_its_own_size_is_told_zero_by_zero_which_keeps_it() {
    let mut fit = Fit::default();
    assert_eq!(fit.again(3), Configure { serial: 3, mode: Mode::Normal, rect: None });
    // Restored from full screen and shown: its own size again.
    let s = fit.ask(Mode::FullScreen, Some(QWEN), FILL, 4).expect("full").serial;
    fit.ack(s);
    fit.shown();
    let s = fit.ask(Mode::Normal, Some(FILL), FILL, 5).expect("back").serial;
    fit.ack(s);
    fit.shown();
    assert_eq!(fit.again(6).size(), (0, 0));
}

#[test]
fn a_filled_window_is_told_its_fill_again() {
    let mut fit = Fit::default();
    let s = fit.ask(Mode::Maximized, Some(QWEN), FILL, 4).expect("max").serial;
    fit.ack(s);
    fit.shown();
    let again = fit.again(7);
    assert_eq!(again, Configure { serial: 7, mode: Mode::Maximized, rect: Some(FILL) });
    // Taken as nothing new: no buffer is due for it.
    fit.ack(7);
    assert_eq!(fit.due(), None);
    assert_eq!(fit.now(), Mode::Maximized);
}

#[test]
fn a_configure_not_yet_acked_takes_the_new_serial() {
    let mut fit = Fit::default();
    assert!(fit.ask(Mode::FullScreen, Some(QWEN), FILL, 4).is_some());
    let again = fit.again(8);
    assert_eq!(again, Configure { serial: 8, mode: Mode::FullScreen, rect: Some(FILL) });
    fit.ack(8);
    assert_eq!(fit.due().map(|c| c.mode), Some(Mode::FullScreen));
}

#[test]
fn an_ack_of_the_first_telling_of_a_configure_told_again_is_taken() {
    // Maximise asked under 4; the window's focus changes before the client
    // read it, so the same mode and size go again under 5. The client acks
    // 4, as it may, and draws for it: that buffer is the maximised one.
    let mut fit = Fit::default();
    assert!(fit.ask(Mode::Maximized, Some(QWEN), FILL, 4).is_some());
    let _ = fit.again(5);
    fit.ack(4);
    let due = fit.due().expect("the maximise is due");
    assert_eq!((due.mode, due.rect), (Mode::Maximized, Some(FILL)));
    // The ack of 5 that follows is of the same configure: nothing new.
    fit.ack(5);
    fit.shown();
    assert_eq!(fit.now(), Mode::Maximized);
    assert_eq!(fit.due(), None);
}

#[test]
fn an_ack_of_a_configure_another_replaced_is_not_taken() {
    let mut fit = Fit::default();
    assert!(fit.ask(Mode::Maximized, Some(QWEN), FILL, 4).is_some());
    let _ = fit.again(5);
    // Restored before the client acked: a configure of another mode.
    assert!(fit.ask(Mode::Normal, Some(FILL), FILL, 6).is_some());
    fit.ack(4);
    fit.ack(5);
    assert_eq!(fit.due(), None, "the maximise it replaced is not taken");
    fit.ack(6);
    assert_eq!(fit.due().map(|c| c.mode), Some(Mode::Normal));
}

#[test]
fn an_ack_ahead_of_the_last_serial_is_not_taken() {
    let mut fit = Fit::default();
    assert!(fit.ask(Mode::FullScreen, Some(QWEN), FILL, 4).is_some());
    fit.ack(9);
    assert_eq!(fit.due(), None, "no configure was told under 9");
}

#[test]
fn the_span_of_serials_holds_across_their_wrap() {
    let mut fit = Fit::default();
    assert!(fit.ask(Mode::FullScreen, Some(QWEN), FILL, u32::MAX).is_some());
    let _ = fit.again(0);
    let _ = fit.again(1);
    fit.ack(u32::MAX - 1);
    assert_eq!(fit.due(), None, "before the first telling");
    fit.ack(0);
    assert_eq!(fit.due().map(|c| c.mode), Some(Mode::FullScreen));
}

#[test]
fn activated_is_said_after_the_mode_states() {
    let told = Configure { serial: 9, mode: Mode::FullScreen, rect: Some(FILL) };
    let mut out = Vec::new();
    configure(30, 29, &told, (true, None), &mut out);
    let w = words(&out);
    assert_eq!(&w[2..7], &[1920, 1022, 8, STATE_FULLSCREEN, STATE_ACTIVATED]);
    assert_eq!(&w[7..10], &[29, 12 << 16, 9]);
    let mut out = Vec::new();
    let own = Configure { mode: Mode::Normal, rect: None, ..told };
    configure(30, 29, &own, (true, None), &mut out);
    assert_eq!(&words(&out)[2..6], &[0, 0, 4, STATE_ACTIVATED]);
    assert_ne!(STATE_MAXIMIZED, STATE_ACTIVATED);
}

#[test]
fn bounds_go_ahead_of_the_configure_when_given() {
    let told = Configure { serial: 9, mode: Mode::Normal, rect: None };
    let mut out = Vec::new();
    configure(30, 29, &told, (false, Some((1280, 708))), &mut out);
    let w = words(&out);
    // configure_bounds: 16 bytes, opcode 2, then the toplevel's configure.
    assert_eq!(&w[0..4], &[30, 16 << 16 | u32::from(TOPLEVEL_CONFIGURE_BOUNDS), 1280, 708]);
    assert_eq!(&w[4..9], &[30, 20 << 16, 0, 0, 0]);
}
