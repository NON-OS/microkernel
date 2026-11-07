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

//! The vector the pack tool writes, read as the device reads it: the list
//! taken by its committed measurement, the head and every member cut out
//! of its chunks, each member decompressed with the device's xz decoder and
//! held to its digest. A byte changed anywhere is refused.

use std::path::PathBuf;

use nonos_app_image::{build, chunk_list, ChunkList, Codec, Head, Part, Role};

use crate::install::image::{assemble, open};

fn vectors() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../nonos_app_image/tests/vectors")
}

pub fn vector() -> (Vec<u8>, Vec<u8>, [u8; 32]) {
    let image = std::fs::read(vectors().join("small.image")).expect("vector image");
    let list = std::fs::read(vectors().join("small.chunks")).expect("vector list");
    let text = std::fs::read_to_string(vectors().join("small.measurement")).expect("measurement");
    let mut pin = [0u8; 32];
    for (i, b) in pin.iter_mut().enumerate() {
        *b = u8::from_str_radix(&text.trim()[2 * i..2 * i + 2], 16).expect("hex");
    }
    (image, list, pin)
}

fn chunks<'a>(list: &'a ChunkList, image: &'a [u8]) -> impl FnMut(usize) -> Option<Vec<u8>> + 'a {
    move |i| {
        let r = list.range(i)?;
        let b = image[r.start as usize..r.end as usize].to_vec();
        list.holds(i, &b).then_some(b)
    }
}

#[test]
fn the_vector_installs_member_by_member() {
    let (image, raw, pin) = vector();
    let list = ChunkList::parse(&raw, &pin).expect("the list is its measurement");
    assert_eq!(list.total, image.len() as u64);
    let mut fetch = chunks(&list, &image);
    let mut get = assemble::Chunks::new(&mut fetch);
    let head = assemble::head(&list, &mut get).expect("head");
    let head = Head::parse(&head).expect("parsed");
    let names: Vec<&[u8]> = head.members.iter().map(|m| m.name).collect();
    assert_eq!(names, [&b"demo"[..], b"libdemo", b"demo-data"]);
    assert_eq!(head.members[0].role, Role::Listed);
    for m in &head.members {
        let stored = assemble::bytes(&list, &m.at, &mut get).expect("member bytes");
        let tar = open::tar(m, stored).expect("decompressed to its digest");
        assert_eq!(tar.len() as u64, m.raw);
        assert_eq!(&tar[257..262], b"ustar");
    }
    assert!(head.members.iter().any(|m| m.codec == Codec::Xz), "the vector carries xz");
}

#[test]
fn a_changed_byte_anywhere_is_refused() {
    let (image, raw, pin) = vector();
    // In the list: its pin no longer holds.
    let mut moved = raw.clone();
    moved[40] ^= 1;
    assert!(ChunkList::parse(&moved, &pin).is_err());
    // In the image: the chunk holding it no longer holds.
    let list = ChunkList::parse(&raw, &pin).expect("list");
    for at in [0, 70, image.len() / 2, image.len() - 1] {
        let mut img = image.clone();
        img[at] ^= 0x40;
        let mut fetch = chunks(&list, &img);
        let mut get = assemble::Chunks::new(&mut fetch);
        let head = assemble::head(&list, &mut get);
        let read = head.as_deref().map(Head::parse);
        let whole = match read {
            Some(Ok(h)) => h.members.iter().all(|m| {
                assemble::bytes(&list, &m.at, &mut get).and_then(|s| open::tar(m, s)).is_some()
            }),
            _ => false,
        };
        assert!(!whole, "a flip at {at} went through");
    }
}

#[test]
fn a_member_whose_stream_is_not_its_tar_is_refused() {
    // The vector's xz stream stored under another tar's digest: decompressed
    // whole, and still refused.
    let (image, _, _) = vector();
    let head = Head::parse(&image).expect("parsed");
    let xz = head.members.iter().find(|m| m.codec == Codec::Xz).expect("an xz member");
    let stream = &image[xz.at.start as usize..xz.at.end as usize];
    let other = vec![7u8; xz.raw as usize];
    let lie = [Part { name: b"liar", codec: Codec::Xz, tar: &other, stored: stream }];
    let img = build(&lie).expect("built");
    let h = Head::parse(&img).expect("parsed");
    let m = &h.members[0];
    assert!(open::tar(m, img[m.at.start as usize..].to_vec()).is_none());
    let garbage = vec![0xfdu8; 40];
    assert!(open::tar(m, garbage).is_none(), "not an xz stream at all");
    let _ = chunk_list(&img, nonos_app_image::CHUNK_MIN).expect("listed");
}

#[test]
fn a_cache_that_holds_every_chunk_asks_for_none_and_one_that_holds_some_asks_for_the_rest() {
    let (_, raw, pin) = vector();
    let list = ChunkList::parse(&raw, &pin).expect("list");
    let all = vec![true; list.count()];
    assert_eq!(nonos_app_image::next(&all, &[], 1), None);
    let none = vec![false; list.count()];
    assert_eq!(nonos_app_image::next(&none, &[], 1), Some(0));
}

#[test]
fn each_chunk_is_read_once_however_many_members_share_it() {
    let (image, raw, pin) = vector();
    let list = ChunkList::parse(&raw, &pin).expect("list");
    let mut reads = vec![0usize; list.count()];
    let mut inner = chunks(&list, &image);
    let mut fetch = |i: usize| {
        reads[i] += 1;
        inner(i)
    };
    let mut get = assemble::Chunks::new(&mut fetch);
    let head_bytes = assemble::head(&list, &mut get).expect("head");
    let head = Head::parse(&head_bytes).expect("parsed");
    for m in &head.members {
        assert!(assemble::bytes(&list, &m.at, &mut get).is_some());
    }
    drop(get);
    assert_eq!(reads, vec![1; list.count()], "three members and a head, one chunk");
}
