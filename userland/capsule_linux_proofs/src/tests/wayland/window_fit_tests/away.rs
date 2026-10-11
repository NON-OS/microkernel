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

//! Full screen asked on the way out, and before the window ever showed.

use super::full::{FILL, QWEN};
use crate::fit::place::centred;
use crate::fit::{origin, Fit, Mode};

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
