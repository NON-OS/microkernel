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

//! Restore, maximise, and asks repeated on the way.

use super::full::{words, FILL, QWEN};
use crate::configure::configure;
use crate::fit::mode::STATE_MAXIMIZED;
use crate::fit::{origin, Fit, Mode};

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
