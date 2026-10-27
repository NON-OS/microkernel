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

//! A selection set, and where a receive on an offer of it goes.
//! Pure, so the host proofs hold it.

use alloc::vec::Vec;

use super::offer::offer_all;
use super::state::{Clip, DESKTOP};
use super::which::Which;
use super::{Event, Objects};

/// The guest's set_selection. `source` is the selection from now, or there
/// is none when it is 0 or not a source of `which`, and `fallback` is
/// offered in its place. The source it replaces is told it is cancelled,
/// unless it is the one set again, and every device is told.
pub fn set(
    objects: &mut Objects,
    clip: &mut Clip,
    (which, source): (Which, u32),
    fallback: Option<u32>,
    out: &mut Vec<u8>,
) -> Option<u32> {
    let new = (source != DESKTOP && objects.get(source) == Some(which.source())).then_some(source);
    let old = core::mem::replace(&mut clip.held[which.at()], new);
    if let Some(old) = old.filter(|&o| Some(o) != new && objects.get(o) == Some(which.source())) {
        Event::new(old, which.source_events().1).send(out);
    }
    offer_all(objects, clip, which, new.or(fallback), out);
    new
}

/// Where a receive on `offer` goes: the guest's source it stands for, while
/// that source is there, or the desktop's clipboard; otherwise nowhere.
pub fn target(objects: &Objects, clip: &Clip, which: Which, offer: u32) -> Option<u32> {
    let source = clip.source_of(offer)?;
    let desktop = source == DESKTOP && which == Which::Data;
    (desktop || objects.get(source) == Some(which.source())).then_some(source)
}

/// A receive forwarded to the guest's own source as its send, the caller
/// having queued the descriptor it carries.
pub fn forward(which: Which, source: u32, mime: &[u8], out: &mut Vec<u8>) {
    Event::new(source, which.source_events().0).string(mime).send(out);
}
