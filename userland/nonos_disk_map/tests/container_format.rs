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

//! The container's own numbers. The digests were computed outside this
//! crate, by a separate FNV-1a implementation over the same offset basis and
//! prime; the empty input gives the offset basis back, as FNV-1a must.

use nonos_disk_map::*;

fn hex(d: [u8; 16]) -> String {
    d.iter().map(|b| format!("{b:02x}")).collect()
}

#[test]
fn digest16_is_fnv1a_128() {
    assert_eq!(hex(digest16(b"")), "6c62272e07bb014262b821756295c58d");
    assert_eq!(hex(digest16(b"a")), "d228cb696f1a8caf78912b704e4a8964");
    assert_eq!(hex(digest16(b"NONOSTR1")), "f8be8e70b3659a76cd841662f8ac9daf");
}

#[test]
fn a_full_store_fits_below_the_plan() {
    let table = HEADER_LEN + ENTRY_LEN * MAX_ENTRIES;
    assert_eq!(TOC_SPAN % SECTOR_SIZE, 0);
    assert!(TOC_SPAN >= table && TOC_SPAN - SECTOR_SIZE < table);
    /*
     * Every payload rounded up to a sector, at most one sector more each.
     */
    let padded = MAX_TOTAL_BYTES + STREAMED_MAX_BYTES + (MAX_ENTRIES * SECTOR_SIZE) as u64;
    let end = STORE_BASE_LBA * SECTOR_SIZE as u64 + TOC_SPAN as u64 + padded;
    assert!(end <= STORE_END_LBA * SECTOR_SIZE as u64);
}

#[test]
fn names_are_ascii_without_nul_and_fit_the_field() {
    assert!(valid_name("/nonos/setup/answers"));
    assert!(valid_name(&format!("/{}", "a".repeat(NAME_LEN - 1))));
    assert!(!valid_name(&format!("/{}", "a".repeat(NAME_LEN))));
    assert!(!valid_name(&"a".repeat(NAME_LEN + 1)));
    assert!(!valid_name(""));
    assert!(!valid_name("/a\0b"));
    assert!(!valid_name("/caf\u{e9}"));
}

/*
 * A name is a path vfs's `normalize` leaves as it is, so a lookup can reach
 * it; nothing that climbs, doubles a slash, or spells a component as a dot.
 */
#[test]
fn names_are_normalized_absolute_paths() {
    for ok in ["/a", "/capsules/x.elf", "/home/My Files/a b.txt", "/a/b/c", "/.hidden", "/a..b"] {
        assert!(valid_name(ok), "{ok}");
    }
    let refused = [
        "a",
        "relative/path",
        "/",
        "//a",
        "/a//b",
        "/a/",
        "/.",
        "/..",
        "/a/./b",
        "/capsules/../nonos/setup/answers",
        "/a/..",
        "/a\nb",
        "/a\tb",
        "/a\x7fb",
        "/a\x1bb",
    ];
    for bad in refused {
        assert!(!valid_name(bad), "{bad:?}");
    }
}
