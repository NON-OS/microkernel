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

//! The chunk list: taken only when it hashes to the pin, each chunk only
//! when it hashes to its row, and a member cut out of its chunks whole.

use super::common;

use common::{image, list};
use nonos_app_image::{chunk_list, covering, ChunkList, Head, Refused, CHUNK_MIN, LIST_HEAD};

#[test]
fn a_list_is_taken_by_its_pin_alone() {
    let img = image();
    let (l, pin) = list(&img, CHUNK_MIN);
    let c = ChunkList::parse(&l, &pin).expect("pinned");
    assert_eq!((c.total, c.chunk), (img.len() as u64, CHUNK_MIN));
    assert_eq!(c.count(), img.len().div_ceil(CHUNK_MIN as usize));
    let mut other = pin;
    other[0] ^= 1;
    assert_eq!(ChunkList::parse(&l, &other).map(|_| ()), Err(Refused::Digest));
    let mut moved = l.clone();
    moved[LIST_HEAD] ^= 1;
    assert_eq!(ChunkList::parse(&moved, &pin).map(|_| ()), Err(Refused::Digest));
}

#[test]
fn every_chunk_holds_its_bytes_and_no_other() {
    let img = image();
    let (l, pin) = list(&img, CHUNK_MIN);
    let c = ChunkList::parse(&l, &pin).expect("pinned");
    for i in 0..c.count() {
        let r = c.range(i).expect("in range");
        let bytes = &img[r.start as usize..r.end as usize];
        assert!(c.holds(i, bytes));
        let mut flipped = bytes.to_vec();
        flipped[bytes.len() / 2] ^= 0x80;
        assert!(!c.holds(i, &flipped));
        assert!(!c.holds(i, &bytes[..bytes.len() - 1]));
    }
    assert!(c.range(c.count()).is_none());
    assert!(!c.holds(c.count(), b""));
}

#[test]
fn a_member_is_cut_out_of_the_chunks_that_cover_it() {
    let img = image();
    let (l, pin) = list(&img, CHUNK_MIN);
    let c = ChunkList::parse(&l, &pin).expect("pinned");
    let head = Head::parse(&img).expect("parsed");
    for m in &head.members {
        let span = covering(&c, &m.at);
        let chunks: Vec<&[u8]> = span
            .clone()
            .map(|i| {
                let r = c.range(i).expect("in range");
                &img[r.start as usize..r.end as usize]
            })
            .collect();
        let cut = c.cut(&m.at, &chunks).expect("cut");
        assert_eq!(cut, &img[m.at.start as usize..m.at.end as usize]);
        assert!(c.cut(&m.at, &chunks[1..]).is_none() || span.len() == 1 && chunks.len() == 1);
    }
    assert!(covering(&c, &(5..5)).is_empty());
    assert!(covering(&c, &(0..c.total + 1)).is_empty());
}

#[test]
fn lists_off_the_layout_are_refused() {
    let img = image();
    let l = chunk_list(&img, CHUNK_MIN).expect("listed");
    let read = |f: &dyn Fn(&mut Vec<u8>)| {
        let mut b = l.clone();
        f(&mut b);
        ChunkList::read(&b).map(|_| ())
    };
    assert_eq!(read(&|b| b[0] = 0), Err(Refused::Magic));
    assert_eq!(read(&|b| b[8] = 2), Err(Refused::Magic));
    assert_eq!(read(&|b| b[30] = 1), Err(Refused::Magic));
    assert_eq!(read(&|b| b[12..16].copy_from_slice(&3u32.to_le_bytes())), Err(Refused::TooLarge));
    assert_eq!(read(&|b| b[12..16].copy_from_slice(&(32u32 << 20).to_le_bytes())), Err(Refused::TooLarge));
    assert_eq!(read(&|b| b[16..24].copy_from_slice(&0u64.to_le_bytes())), Err(Refused::TooLarge));
    assert_eq!(read(&|b| b.push(0)), Err(Refused::Layout));
    assert_eq!(read(&|b| b.truncate(b.len() - 1)), Err(Refused::Layout));
    assert_eq!(chunk_list(&img, 1000).map(|_| ()), Err(Refused::Build));
    assert_eq!(chunk_list(&[], CHUNK_MIN).map(|_| ()), Err(Refused::Build));
}
