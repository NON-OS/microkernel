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

//! Walking a tar, which is what a package is once it is decompressed.
//!
//! Every typeflag a package carries is kept: files, symbolic and hard links,
//! directories, and the pax and GNU records that give a long path. Devices
//! and fifos are counted as dropped, so a lost entry is a number, not silence.

use alloc::vec::Vec;

use super::tar_field::octal;
use super::tar_kind::{read, Read};
use super::tar_pax::Overrides;

pub use super::tar_kind::{Entry, Kind};

const BLOCK: usize = 512;
const SIZE_AT: usize = 124;
const SIZE_LEN: usize = 12;
const TYPE_AT: usize = 156;

/// Everything in the archive, and how many entries had nowhere to go.
pub struct Walk {
    pub entries: Vec<Entry>,
    pub dropped: u32,
}

/// Regular files only, for the readers that want a named file's bytes.
pub fn entries(data: &[u8]) -> Vec<Entry> {
    walk(data).entries.into_iter().filter(|e| matches!(e.kind, Kind::File)).collect()
}

pub fn walk(data: &[u8]) -> Walk {
    let mut out = Walk { entries: Vec::new(), dropped: 0 };
    let mut next = Overrides::default();
    let mut at = 0usize;
    while at + BLOCK <= data.len() {
        let head = &data[at..at + BLOCK];
        // A zero block ends an archive, and an apk is several end to end.
        if head.iter().all(|b| *b == 0) {
            at += BLOCK;
            continue;
        }
        let Some(size) = octal(&head[SIZE_AT..SIZE_AT + SIZE_LEN]) else {
            break;
        };
        let body_at = at + BLOCK;
        let Some(end) = body_at.checked_add(size).filter(|e| *e <= data.len()) else {
            break;
        };
        match read(head[TYPE_AT], head, &data[body_at..end], &mut next) {
            Read::Entry(e) => out.entries.push(e),
            Read::Record => {}
            Read::Dropped => out.dropped += 1,
        }
        at = body_at + size.div_ceil(BLOCK) * BLOCK;
    }
    out
}
