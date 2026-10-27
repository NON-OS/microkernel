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

//! The ids of the offers a guest is given, and the mime types a source is
//! held to: a freed server id is given again, lowest first, and a source
//! offers no more than the bounds keep.

use super::clip_wire::{guest, SOURCES};
use crate::wayland_clip::offer::{server_id, SERVER_START};
use crate::wayland_clip::set::set;
use crate::wayland_clip::which::Which;

#[test]
fn a_freed_server_id_is_given_again_lowest_first() {
    let (mut objects, mut clip) = guest();
    let mut out = Vec::new();
    set(&mut objects, &mut clip, (Which::Data, SOURCES[0]), None, &mut out);
    crate::wayland_clip::gone::offer_gone(&mut objects, &mut clip, SERVER_START);
    assert!(objects.get(SERVER_START).is_none());
    assert_eq!(server_id(&objects), SERVER_START);
    assert_eq!(clip.source_of(SERVER_START), None);
    assert_eq!(clip.source_of(SERVER_START + 1), Some(SOURCES[0]));
}

#[test]
fn a_source_keeps_its_mime_types_bounded() {
    let (_, mut clip) = guest();
    clip.add_mime(SOURCES[0], b"image/png");
    assert_eq!(clip.mimes_of(SOURCES[0]).len(), 2, "a mime type offered twice is kept once");
    clip.add_mime(SOURCES[0], &[b'x'; 257]);
    (0..100).for_each(|n| clip.add_mime(SOURCES[0], format!("a/{n}").as_bytes()));
    assert_eq!(clip.mimes_of(SOURCES[0]).len(), 64, "too long, then too many, are not kept");
}
