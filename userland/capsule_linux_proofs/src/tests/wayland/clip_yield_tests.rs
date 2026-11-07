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

//! A guest's clipboard selection gives way to text copied on the desktop
//! since: Ctrl+V in Telegram after a copy in another window pastes that
//! copy, not the text Telegram copied earlier.

use super::clip_replace_tests::bare;
use super::clip_wire::{guest, SOURCES};
use crate::ops::data::*;
use crate::wayland_clip::desk_yield::{desk_newer, digest, give_way};
use crate::wayland_clip::set::set;
use crate::wayland_clip::which::Which;

const FOCUS: &str = include_str!("../../../../capsule_linux/src/linux/wayland/clip/focus.rs");
const COPY: &str = include_str!("../../../../capsule_linux/src/linux/wayland/clip/desk_copy.rs");

#[test]
fn other_text_on_the_desktop_is_newer_than_the_guests() {
    let ours = Some(digest(b"from telegram"));
    assert!(desk_newer(ours, b"from the browser"));
    assert!(!desk_newer(ours, b"from telegram"), "still the guest's own");
    assert!(!desk_newer(ours, b""), "an empty clipboard takes nothing away");
    assert!(!desk_newer(None, b"anything"), "a selection never copied out is kept");
}

#[test]
fn a_selection_that_gives_way_is_cancelled_and_none_is_held() {
    let (mut objects, mut clip) = guest();
    let mut out = Vec::new();
    set(&mut objects, &mut clip, (Which::Data, SOURCES[0]), None, &mut out);
    clip.copied = Some(digest(b"old"));
    let primary = clip.held[Which::Primary.at()];
    let mut ended = Vec::new();
    give_way(&objects, &mut clip, &mut ended);
    assert_eq!(bare(&ended), vec![super::clip_wire::Ev::Bare(SOURCES[0], SOURCE_CANCELLED)]);
    assert_eq!(clip.held[Which::Data.at()], None);
    assert_eq!(clip.held[Which::Primary.at()], primary, "the primary selection stays");
    assert_eq!(clip.copied, None);
    let mut again = Vec::new();
    give_way(&objects, &mut clip, &mut again);
    assert!(again.is_empty(), "cancelled once");
}

#[test]
fn the_keyboard_enter_asks_before_it_offers() {
    let at = FOCUS.find("pub fn entered(").expect("entered");
    let body = &FOCUS[at..];
    let ask = body.find("yield_to_desk(guest)").expect("asks the desktop");
    let offer = body.find("offer_all(").expect("offers");
    assert!(ask < offer, "the selection given way to is the one offered");
    // The desktop is read only for a guest that copied to it.
    let at = COPY.find("pub fn yield_to_desk(").expect("yield_to_desk");
    let read = COPY[at..].find("clipboard_paste(").expect("reads");
    assert!(COPY[at..at + read].contains("copied.is_none()"));
}
