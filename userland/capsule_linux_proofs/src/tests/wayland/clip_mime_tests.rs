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

//! The desktop path's text types: only text is read from a guest's
//! selection or offered from the desktop, UTF-8 first, and the desktop's
//! offer is the clipboard's alone.

use super::clip_wire::{events, guest, Ev, DEVICES, SOURCES};
use crate::ops::data::*;
use crate::wayland_clip::mime::{is_text, text_of, TEXT};
use crate::wayland_clip::offer::SERVER_START;
use crate::wayland_clip::set::{set, target};
use crate::wayland_clip::state::DESKTOP;
use crate::wayland_clip::which::Which;

fn mimes(all: &[&str]) -> Vec<Vec<u8>> {
    all.iter().map(|m| m.as_bytes().to_vec()).collect()
}

#[test]
fn only_text_types_are_carried() {
    assert!(TEXT.iter().all(|t| is_text(t)));
    assert!(is_text(b"text/plain;charset=UTF-8"), "a charset in either case");
    for other in ["image/png", "text/html", "text/uri-list", "application/x-kde-cutselection", ""] {
        assert!(!is_text(other.as_bytes()), "{other}");
    }
    assert_eq!(text_of(&mimes(&["image/png", "text/html"])), None);
}

#[test]
fn a_selection_is_read_as_utf8_text_when_it_offers_it() {
    let offered = mimes(&["STRING", "text/plain", "image/png", "text/plain;charset=UTF-8"]);
    assert_eq!(text_of(&offered), Some(&b"text/plain;charset=UTF-8"[..]), "its own spelling");
    assert_eq!(text_of(&mimes(&["TEXT", "UTF8_STRING"])), Some(&b"UTF8_STRING"[..]));
    assert_eq!(text_of(&mimes(&["STRING"])), Some(&b"STRING"[..]));
}

#[test]
fn with_no_guest_selection_the_desktop_offers_text_alone() {
    let (mut objects, mut clip) = guest();
    let mut out = Vec::new();
    let fallback = Some(DESKTOP);
    assert_eq!(set(&mut objects, &mut clip, (Which::Data, 0), fallback, &mut out), None);
    let sent = events(&out, |o, _| o >= SERVER_START);
    let offered: Vec<&[u8]> = sent[1..6]
        .iter()
        .map(|e| match e {
            Ev::Text(SERVER_START, OFFER_OFFER, m) => m.as_slice(),
            other => panic!("{other:?}"),
        })
        .collect();
    assert_eq!(offered, TEXT.to_vec());
    assert_eq!(sent[6], Ev::Word(DEVICES[0], DEVICE_SELECTION, SERVER_START));
    assert_eq!(target(&objects, &clip, Which::Data, SERVER_START), Some(DESKTOP));
    assert_eq!(target(&objects, &clip, Which::Primary, SERVER_START), None);
    // The guest's own selection takes the desktop's place; none is cancelled.
    let mut own = Vec::new();
    set(&mut objects, &mut clip, (Which::Data, SOURCES[0]), fallback, &mut own);
    let sent = events(&own, |o, _| o >= SERVER_START);
    assert!(!sent.iter().any(|e| matches!(e, Ev::Bare(..))));
}
