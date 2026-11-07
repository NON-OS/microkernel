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

//! A real image the pack tool built, when NONOS_APP_IMAGE names its
//! `.image` file (its `.chunks` beside it): read as the device reads it, with
//! the time each step took on this host. Ignored by default: the image is
//! large and fetched from Alpine, and is not kept in the tree.
//!
//!     NONOS_APP_IMAGE=/path/<pin>.image cargo test --release -- --ignored --nocapture

use std::time::Instant;

use nonos_app_image::{ChunkList, Head};

use crate::install::image::{assemble, open};

#[test]
#[ignore]
fn a_real_image_reads_whole_and_says_how_long() {
    let Ok(path) = std::env::var("NONOS_APP_IMAGE") else { return };
    let image = std::fs::read(&path).expect("image");
    let raw = std::fs::read(path.replace(".image", ".chunks")).expect("chunks");
    let pin = *blake3::hash(&raw).as_bytes();
    let t = Instant::now();
    let list = ChunkList::parse(&raw, &pin).expect("list");
    let mut fetch = |i: usize| {
        let r = list.range(i)?;
        let b = image[r.start as usize..r.end as usize].to_vec();
        list.holds(i, &b).then_some(b)
    };
    let mut get = assemble::Chunks::new(&mut fetch);
    let count = Head::parse(&assemble::head(&list, &mut get).expect("head")).map(|h| h.members.len());
    let checked = t.elapsed();
    let head_bytes = assemble::head(&list, &mut get).expect("head");
    let head = Head::parse(&head_bytes).expect("parsed");
    let (mut raw_total, t) = (0u64, Instant::now());
    for m in &head.members {
        let stored = assemble::bytes(&list, &m.at, &mut get).expect("bytes");
        let tar = open::tar(m, stored).expect("member");
        raw_total += tar.len() as u64;
    }
    let opened = t.elapsed();
    println!(
        "image {} bytes, {} members, {} raw bytes: list and head {:.1} ms; chunks re-hashed, \
         members cut, decompressed and held to their digests {:.2} s (host)",
        image.len(),
        head.members.len(),
        raw_total,
        checked.as_secs_f64() * 1e3,
        opened.as_secs_f64()
    );
    assert!(count.is_ok_and(|n| n > 0));
}
