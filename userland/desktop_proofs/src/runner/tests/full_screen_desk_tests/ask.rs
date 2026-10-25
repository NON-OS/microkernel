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

//! An app's ask for full screen, which never undoes the person's choice.

use super::super::super::full_screen_ask::{FullScreenAsk, Turn};

/// An app that asks from the start opens full screen; one that stops asking
/// gets its rect back.
#[test]
fn an_app_asking_from_its_first_frame_opens_full_screen_and_gives_it_back() {
    let mut ask = FullScreenAsk::new();
    assert_eq!(ask.step(true, false), Turn::Enter);
    assert_eq!(ask.step(true, true), Turn::Stay, "asked once, taken once");
    assert_eq!(ask.step(false, true), Turn::Leave);
    assert_eq!(ask.step(false, false), Turn::Stay);
}

/// Video playing asks; the person made the window full screen first with
/// green, so the end of playback leaves it as they chose.
#[test]
fn the_ask_never_undoes_what_the_person_chose() {
    let mut ask = FullScreenAsk::new();
    // Full screen by green, then the app asks and stops asking.
    assert_eq!(ask.step(true, true), Turn::Stay);
    assert_eq!(ask.step(false, true), Turn::Stay);

    // Taken by the ask, then green pressed twice by the person (out and back
    // in): the person's full screen stays when the ask ends.
    let mut ask = FullScreenAsk::new();
    assert_eq!(ask.step(true, false), Turn::Enter);
    ask.person_chose();
    assert_eq!(ask.step(false, true), Turn::Stay);

    // Taken by the ask, left by green: the ask ending changes nothing, and
    // asking again takes it again.
    let mut ask = FullScreenAsk::new();
    assert_eq!(ask.step(true, false), Turn::Enter);
    ask.person_chose();
    assert_eq!(ask.step(false, false), Turn::Stay);
    assert_eq!(ask.step(true, false), Turn::Enter);
}
