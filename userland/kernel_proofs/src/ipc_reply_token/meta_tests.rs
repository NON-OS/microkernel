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

use super::recv_meta::{meta_len_ok, meta_v1, META_V1, META_V1_LEN};
use super::request_ids::{forget, next};

/// Each server's ids start at 1, never repeat while it lives, and are its
/// own; a dead server's go, so the next process with its pid starts over.
#[test]
fn request_ids_are_per_server_from_1_and_never_reused() {
    let (a, b) = (0x7e57_0001, 0x7e57_0002);
    assert_eq!([next(a), next(a), next(a)], [1, 2, 3]);
    assert_eq!(next(b), 1, "another server's count is not shared");
    forget(a);
    assert_eq!(next(a), 1);
    assert_eq!(next(b), 2);
}

/// Only the v1 length is taken: a shorter meta would be cut, a longer one
/// is another version's.
#[test]
fn only_the_v1_meta_length_is_taken() {
    assert!(meta_len_ok(24));
    for len in [0, 8, 16, 23, 25, 32, u64::MAX] {
        assert!(!meta_len_ok(len), "{len}");
    }
}

/// The meta v1 bytes: version, reserved, sender, enqueued_ms, request,
/// little-endian, at the offsets abi/wire.toml pins.
#[test]
fn meta_v1_is_the_wire_files_layout() {
    let m = meta_v1(0x0403_0201, 0x0c0b_0a09_0807_0605, 0x1413_1211_100f_0e0d);
    assert_eq!(m.len(), META_V1_LEN);
    let wire = std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/../../abi/wire.toml"))
        .unwrap();
    let section = wire.split("[ipc.recv_meta_v1]").nth(1).unwrap();
    let field = |name: &str| -> (usize, usize) {
        let line = section.lines().find(|l| l.starts_with(&format!("{name} "))).unwrap();
        let offset = line.split("offset = ").nth(1).unwrap().split(',').next().unwrap();
        let size = match line.split("type = \"").nth(1).unwrap().split('"').next().unwrap() {
            "u16" => 2,
            "u32" => 4,
            _ => 8,
        };
        (offset.trim().parse().unwrap(), size)
    };
    assert!(section.contains(&format!("size_bytes = {META_V1_LEN}\n")));
    let le = |(at, size): (usize, usize)| {
        m[at..at + size].iter().rev().fold(0u64, |v, b| v << 8 | *b as u64)
    };
    assert_eq!(le(field("version")), META_V1 as u64);
    assert_eq!(le(field("reserved")), 0);
    assert_eq!(le(field("sender")), 0x0403_0201);
    assert_eq!(le(field("enqueued_ms")), 0x0c0b_0a09_0807_0605);
    assert_eq!(le(field("request")), 0x1413_1211_100f_0e0d);
}
