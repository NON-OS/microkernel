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

//! A selection replaced: the source it was is cancelled, once, and only
//! when another takes its place; a source destroyed takes its selection
//! with it.

use super::clip_wire::{events, guest, Ev, DEVICES, SOURCES};
use crate::ops::data::*;
use crate::wayland_clip::gone::source_gone;
use crate::wayland_clip::offer::SERVER_START;
use crate::wayland_clip::set::set;
use crate::wayland_clip::which::Which;

pub(super) fn bare(out: &[u8]) -> Vec<Ev> {
    let all = events(out, |o, _| o >= SERVER_START);
    all.into_iter().filter(|e| matches!(e, Ev::Bare(..))).collect()
}

#[test]
fn a_new_selection_cancels_the_one_it_replaces() {
    let (mut objects, mut clip) = guest();
    let (mut first, mut second, mut again) = (Vec::new(), Vec::new(), Vec::new());
    set(&mut objects, &mut clip, (Which::Data, SOURCES[0]), None, &mut first);
    assert!(bare(&first).is_empty(), "nothing held before, so nothing is cancelled");
    set(&mut objects, &mut clip, (Which::Data, SOURCES[1]), None, &mut second);
    assert_eq!(bare(&second), vec![Ev::Bare(SOURCES[0], SOURCE_CANCELLED)]);
    set(&mut objects, &mut clip, (Which::Data, SOURCES[1]), None, &mut again);
    assert!(bare(&again).is_empty(), "a source set again is not cancelled");
    let mut cleared = Vec::new();
    assert_eq!(set(&mut objects, &mut clip, (Which::Data, 0), None, &mut cleared), None);
    let mut want = vec![Ev::Bare(SOURCES[1], SOURCE_CANCELLED)];
    want.extend(DEVICES.map(|d| Ev::Word(d, DEVICE_SELECTION, 0)));
    assert_eq!(events(&cleared, |_, _| false), want);
}

#[test]
fn a_destroyed_selection_leaves_none() {
    let (mut objects, mut clip) = guest();
    let mut out = Vec::new();
    set(&mut objects, &mut clip, (Which::Data, SOURCES[0]), None, &mut out);
    let mut gone = Vec::new();
    source_gone(&mut objects, &mut clip, (Which::Data, SOURCES[1]), None, &mut gone);
    assert!(gone.is_empty(), "a source that was not the selection changes nothing");
    source_gone(&mut objects, &mut clip, (Which::Data, SOURCES[0]), None, &mut gone);
    let want: Vec<Ev> = DEVICES.map(|d| Ev::Word(d, DEVICE_SELECTION, 0)).into();
    assert_eq!(events(&gone, |_, _| false), want);
    assert_eq!(clip.held, [None, None]);
}
