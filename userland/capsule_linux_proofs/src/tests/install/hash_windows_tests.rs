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


//! A shipped tier's program checked against its pin a window at a time: the
//! same hash as the whole file, with no allocation of its full size.

use crate::hash_windows::hash_windows;

fn program(len: usize) -> Vec<u8> {
    (0..len).map(|i| (i * 31 + i / 4096) as u8).collect()
}

fn windows(data: &[u8], window: u32) -> Result<[u8; 32], &'static str> {
    hash_windows(data.len() as u64, window, "early", |at, len| {
        let at = at as usize;
        Ok(data[at..(at + len as usize).min(data.len())].to_vec())
    })
}

#[test]
fn hashed_a_window_at_a_time_it_is_the_hash_of_the_whole_program() {
    for len in [0usize, 1, 4095, 1 << 20, (1 << 20) + 1, 5 * (1 << 20) + 12_345] {
        let data = program(len);
        let whole = *blake3::hash(&data).as_bytes();
        for window in [1u32 << 20, 4096, 777] {
            assert_eq!(windows(&data, window), Ok(whole), "{len} bytes in {window}");
        }
    }
}

#[test]
fn no_window_is_larger_than_asked_however_large_the_program() {
    let data = program(14 * (1 << 20));
    let mut largest = 0u32;
    let got = hash_windows(data.len() as u64, 1 << 20, "early", |at, len| {
        largest = largest.max(len);
        let at = at as usize;
        Ok::<_, &str>(data[at..at + len as usize].to_vec())
    });
    assert_eq!(got, Ok(*blake3::hash(&data).as_bytes()));
    assert_eq!(largest, 1 << 20, "a 14 MB program is read a mebibyte at a time");
}

#[test]
fn a_store_that_ends_early_or_fails_says_so_rather_than_hashing_part() {
    let data = program(3 * 4096);
    let short = hash_windows(data.len() as u64 + 10, 4096, "early", |at, len| {
        let at = at as usize;
        Ok(data[at.min(data.len())..(at + len as usize).min(data.len())].to_vec())
    });
    assert_eq!(short, Err("early"));
    let failed = hash_windows(data.len() as u64, 4096, "early", |at, _| {
        if at >= 4096 { Err("the store refused the read") } else { Ok(vec![0u8; 4096]) }
    });
    assert_eq!(failed, Err("the store refused the read"));
}
