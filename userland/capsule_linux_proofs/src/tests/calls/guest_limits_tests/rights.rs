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

//! SCM_RIGHTS read from every rights block, and a walk that never steps back.

use crate::linux::unix::cmsg::{rights, MAX_FDS};

fn block(level: u32, kind: u32, fds: &[u32]) -> Vec<u8> {
    let len = 16 + 4 * fds.len();
    let mut b = Vec::new();
    b.extend_from_slice(&(len as u64).to_le_bytes());
    b.extend_from_slice(&level.to_le_bytes());
    b.extend_from_slice(&kind.to_le_bytes());
    fds.iter().for_each(|fd| b.extend_from_slice(&fd.to_le_bytes()));
    b.resize((len + 7) & !7, 0);
    b
}

#[test]
fn scm_rights_descriptors_are_read_from_every_rights_block_and_nothing_else() {
    let mut raw = block(1, 1, &[3, 4, 5]);
    raw.extend(block(1, 2, &[9, 9])); // SCM_CREDENTIALS-shaped: stepped over
    raw.extend(block(6, 1, &[9])); // another level: stepped over
    raw.extend(block(1, 1, &[7]));
    assert_eq!(rights(&raw), [3, 4, 5, 7]);
    let many: Vec<u32> = (0..300).collect();
    assert_eq!(rights(&block(1, 1, &many)).len(), MAX_FDS, "SCM_MAX_FD");
}

#[test]
fn a_length_that_steps_back_or_past_the_end_ends_the_walk() {
    // A second header whose length, added to where it starts, wraps round to
    // the first: the old walk went back to the top of the block for ever.
    let mut back = block(0, 0, &[]);
    back.extend_from_slice(&(u64::MAX - 15).to_le_bytes());
    back.extend_from_slice(&[0; 8]);
    assert_eq!(rights(&back), [] as [u32; 0]);
    let good = block(1, 1, &[3]);
    let mut raw = good.clone();
    raw.extend_from_slice(&(u64::MAX - 10).to_le_bytes());
    raw.extend_from_slice(&[1, 0, 0, 0, 1, 0, 0, 0]);
    assert_eq!(rights(&raw), [3]);
    // Shorter than a header, longer than what is left.
    let mut short = good.clone();
    short.extend_from_slice(&8u64.to_le_bytes());
    short.extend_from_slice(&[1, 0, 0, 0, 1, 0, 0, 0]);
    assert_eq!(rights(&short), [3]);
    let mut long = good;
    long.extend_from_slice(&64u64.to_le_bytes());
    long.extend_from_slice(&[1, 0, 0, 0, 1, 0, 0, 0, 9, 0, 0, 0]);
    assert_eq!(rights(&long), [3]);
}
