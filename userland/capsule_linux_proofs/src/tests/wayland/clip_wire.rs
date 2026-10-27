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

//! The events the selection code writes, read back off the wire, and the
//! objects a guest with two devices and two sources of each kind holds.

use crate::args::Args;
use crate::wayland_clip::state::Clip;
use crate::wayland_object::{Object, Objects};
use crate::wire::next;

/// Two clipboard devices, two primary devices, two sources of each.
pub const DEVICES: [u32; 2] = [10, 11];
pub const PRIMARY_DEVICES: [u32; 2] = [12, 13];
pub const SOURCES: [u32; 2] = [20, 21];
pub const PRIMARY_SOURCES: [u32; 2] = [30, 31];

/// One event: the object, the opcode, and its first word or string.
#[derive(Debug, PartialEq, Eq)]
pub enum Ev {
    Word(u32, u16, u32),
    Text(u32, u16, Vec<u8>),
    Bare(u32, u16),
}

/// Every event in `out`; one `strings` names is read as its string.
pub fn events(out: &[u8], strings: impl Fn(u32, u16) -> bool) -> Vec<Ev> {
    let mut at = 0;
    let mut all = Vec::new();
    while let Some((msg, size)) = next(&out[at..]) {
        let mut args = Args::new(msg.args);
        all.push(match (msg.args.is_empty(), strings(msg.object, msg.opcode)) {
            (true, _) => Ev::Bare(msg.object, msg.opcode),
            (false, true) => Ev::Text(msg.object, msg.opcode, args.string().unwrap().to_vec()),
            (false, false) => Ev::Word(msg.object, msg.opcode, args.u32().unwrap()),
        });
        at += size;
    }
    assert_eq!(at, out.len(), "every byte is a whole event");
    all
}

/// The guest: a seat, its devices at version 3, and sources offering text
/// and a picture.
pub fn guest() -> (Objects, Clip) {
    let mut objects = Objects::new();
    assert!(objects.bound(5, Object::Seat, 5));
    let mut clip = Clip::default();
    let kinds = [(DEVICES, Object::DataDevice), (PRIMARY_DEVICES, Object::PrimaryDevice)];
    for (ids, kind) in kinds {
        ids.iter().for_each(|&id| assert!(objects.bound(id, kind, 3)));
    }
    let kinds = [(SOURCES, Object::DataSource), (PRIMARY_SOURCES, Object::PrimarySource)];
    for (ids, kind) in kinds {
        for id in ids {
            assert!(objects.bound(id, kind, 3));
            clip.add_mime(id, b"text/plain;charset=utf-8");
            clip.add_mime(id, b"image/png");
        }
    }
    (objects, clip)
}
