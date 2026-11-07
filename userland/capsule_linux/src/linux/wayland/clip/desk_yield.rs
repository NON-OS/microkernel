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

//! A guest's clipboard selection giving way to the desktop's. Nothing tells
//! the bridge the desktop clipboard changed, so a guest that copied once
//! would keep pasting its own old text after the person copied something
//! else in another window. As the keyboard enters, the desktop's text is
//! compared with what this guest copied there: when the desktop holds other
//! text, the guest's selection ends and the desktop's is offered. Pure, so
//! the host proofs hold it.

use alloc::vec::Vec;

use super::state::Clip;
use super::which::Which;
use super::{Event, Objects};

/// FNV-1a, enough to tell the text this guest copied from any other.
pub fn digest(bytes: &[u8]) -> u64 {
    bytes.iter().fold(0xcbf2_9ce4_8422_2325, |h, &b| (h ^ b as u64).wrapping_mul(0x100_0000_01b3))
}

/// Whether text was copied to the desktop since this guest copied
/// `copied` there: it holds `desk`, which is text and not the guest's. A
/// selection the guest never copied there (no focus, not text, too long)
/// is kept, since nothing says which is newer.
pub fn desk_newer(copied: Option<u64>, desk: &[u8]) -> bool {
    copied.is_some_and(|ours| !desk.is_empty() && digest(desk) != ours)
}

/// The guest's clipboard selection ends: its source is told it is
/// cancelled, as a source replaced is (set.rs), and nothing of it is kept.
/// The caller offers the desktop's in its place.
pub fn give_way(objects: &Objects, clip: &mut Clip, out: &mut Vec<u8>) {
    let which = Which::Data;
    clip.copied = None;
    let Some(old) = clip.held[which.at()].take() else { return };
    if objects.get(old) == Some(which.source()) {
        Event::new(old, which.source_events().1).send(out);
    }
}
