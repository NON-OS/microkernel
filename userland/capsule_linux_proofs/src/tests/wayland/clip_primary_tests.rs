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

//! The primary selection, which is offered to the guest's primary
//! selection devices and replaced as the clipboard selection is.

use super::clip_replace_tests::bare;
use super::clip_wire::{events, guest, Ev, PRIMARY_DEVICES, PRIMARY_SOURCES, SOURCES};
use crate::ops::select::*;
use crate::wayland_clip::offer::SERVER_START;
use crate::wayland_clip::set::set;
use crate::wayland_clip::which::Which;

#[test]
fn the_primary_selection_is_offered_and_replaced_the_same_way() {
    let (mut objects, mut clip) = guest();
    let mut out = Vec::new();
    set(&mut objects, &mut clip, (Which::Primary, PRIMARY_SOURCES[0]), None, &mut out);
    let sent = events(&out, |o, _| o >= SERVER_START);
    let offer = Ev::Word(PRIMARY_DEVICES[0], PRIMARY_DEVICE_DATA_OFFER, SERVER_START);
    assert_eq!(sent[0], offer);
    let selection = Ev::Word(PRIMARY_DEVICES[0], PRIMARY_DEVICE_SELECTION, SERVER_START);
    assert_eq!(sent[3], selection);
    assert_eq!(sent.len(), 8, "both primary devices, and no clipboard device");
    let mut next = Vec::new();
    set(&mut objects, &mut clip, (Which::Primary, PRIMARY_SOURCES[1]), None, &mut next);
    assert_eq!(bare(&next), vec![Ev::Bare(PRIMARY_SOURCES[0], PRIMARY_SOURCE_CANCELLED)]);
    // A clipboard source is not a primary one, so it sets nothing there.
    let mut wrong = Vec::new();
    assert_eq!(set(&mut objects, &mut clip, (Which::Primary, SOURCES[0]), None, &mut wrong), None);
}
