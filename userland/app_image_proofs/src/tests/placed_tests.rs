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

//! The record of what an image install placed: read back exactly as kept,
//! for its own pin only, and refused whole when it is anything else. With
//! it a re-install checks one head per pack and decodes nothing.

use std::time::Instant;

use crate::install::image::placed::{decode, encode, Placed, MAX, MAX_BYTES};

fn packs(n: usize) -> Vec<Placed> {
    (0..n).map(|i| Placed { name: format!("pkg-{i}"), head: [i as u8; 32] }).collect()
}

#[test]
fn a_record_reads_back_for_its_own_pin_only() {
    let (pin, other) = ([7u8; 32], [8u8; 32]);
    let p = packs(253);
    let raw = encode(&pin, &p).expect("encoded");
    assert_eq!(decode(&raw, &pin), Some(p));
    assert_eq!(decode(&raw, &other), None);
}

#[test]
fn a_malformed_record_is_refused_whole() {
    let pin = [1u8; 32];
    let raw = encode(&pin, &packs(3)).expect("encoded");
    let mut longer = raw.clone();
    longer.push(0);
    assert_eq!(decode(&longer, &pin), None, "bytes after the last pack");
    assert_eq!(decode(&raw[..raw.len() - 1], &pin), None, "cut short");
    let mut magic = raw.clone();
    magic[0] = b'X';
    assert_eq!(decode(&magic, &pin), None);
    let mut more = raw.clone();
    more[40] = 4;
    assert_eq!(decode(&more, &pin), None, "a count past its packs");
    let mut zero = raw.clone();
    zero[40] = 0;
    zero[41] = 0;
    assert_eq!(decode(&zero, &pin), None);
    for cut in 0..raw.len() {
        let _ = decode(&raw[..cut], &pin);
    }
}

#[test]
fn what_cannot_be_read_back_is_not_written() {
    let pin = [0u8; 32];
    assert_eq!(encode(&pin, &[]), None);
    assert_eq!(encode(&pin, &packs(MAX + 1)), None);
    let slash = [Placed { name: "a/b".into(), head: [0; 32] }];
    assert_eq!(encode(&pin, &slash), None);
    let empty = [Placed { name: String::new(), head: [0; 32] }];
    assert_eq!(encode(&pin, &empty), None);
    let long = [Placed { name: "a".repeat(256), head: [0; 32] }];
    assert_eq!(encode(&pin, &long), None);
    let full: Vec<Placed> = (0..MAX).map(|i| Placed { name: format!("{i:0>255}"), head: [0; 32] }).collect();
    let raw = encode(&pin, &full).expect("the largest record");
    assert_eq!(raw.len(), MAX_BYTES);
    assert!(decode(&raw, &pin).is_some());
}

/// The host share of a re-install of Telegram's image: its 253 packs' heads
/// hashed against the record. The store reads and the trailer checks are
/// the device's and are not timed here.
#[test]
#[ignore]
fn a_reinstall_check_on_the_host() {
    let Ok(sizes) = std::env::var("NONOS_PACK_HEADS") else { return };
    let heads: Vec<Vec<u8>> = sizes.split(',').filter_map(|s| s.parse::<usize>().ok()).map(|n| vec![3u8; n]).collect();
    let pin = [9u8; 32];
    let p: Vec<Placed> = heads
        .iter()
        .enumerate()
        .map(|(i, h)| Placed { name: format!("pkg-{i}"), head: *blake3::hash(h).as_bytes() })
        .collect();
    let raw = encode(&pin, &p).expect("encoded");
    let t = Instant::now();
    let back = decode(&raw, &pin).expect("decoded");
    let ok = back.iter().zip(&heads).all(|(r, h)| *blake3::hash(h).as_bytes() == r.head);
    let took = t.elapsed();
    let bytes: usize = heads.iter().map(Vec::len).sum();
    println!("{} packs, {} head bytes: record decoded and every head hashed in {:.2} ms (host)", heads.len(), bytes, took.as_secs_f64() * 1e3);
    assert!(ok);
}
