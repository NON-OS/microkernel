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

//! What the installer's disk list says a disk holds, from its first sectors
//! (`nonos_blk_client/src/disks/contents.rs`). On the HP (6 Oct) the drive
//! stopped answering, its read failed with -110, the list said
//! "unrecognised contents" and offered it, and the install stalled at 2%.
//! A failed read is its own answer now, with the driver's status.

#[path = "../../nonos_blk_client/src/disks/contents.rs"]
mod contents;

use contents::{Contents, HEAD};

const DISK: u64 = 500_118_192;

fn head_with(fill: impl FnOnce(&mut [u8; HEAD])) -> [u8; HEAD] {
    let mut h = [0u8; HEAD];
    fill(&mut h);
    h
}

fn gpt_named(name: &str) -> [u8; HEAD] {
    head_with(|h| {
        h[512..520].copy_from_slice(b"EFI PART");
        for (i, u) in name.encode_utf16().enumerate() {
            h[1024 + 56 + i * 2..1024 + 58 + i * 2].copy_from_slice(&u.to_le_bytes());
        }
    })
}

#[test]
fn a_read_that_fails_is_unread_with_the_drivers_status_never_unrecognised() {
    for status in [-110, -5, -0x1281, -19] {
        let got = Contents::read_with(DISK, |_| Err(status));
        assert_eq!(got, Contents::Unread(status));
        assert_ne!(got, Contents::Unknown);
    }
}

#[test]
fn a_read_that_succeeds_says_what_the_sectors_hold() {
    let read = |h: [u8; HEAD]| {
        Contents::read_with(DISK, move |out| {
            *out = h;
            Ok(())
        })
    };
    assert_eq!(read(gpt_named("NONOS-ESP")), Contents::Nonos);
    assert_eq!(read(gpt_named("NONOS-STORE")), Contents::Nonos);
    assert_eq!(read(gpt_named("EFI system partition")), Contents::OtherGpt);
    let mbr = head_with(|h| (h[446 + 4], h[510], h[511]) = (0x07, 0x55, 0xAA));
    assert_eq!(read(mbr), Contents::Mbr);
    assert_eq!(read([0u8; HEAD]), Contents::Blank);
    assert_eq!(read(head_with(|h| h[700] = 1)), Contents::Unknown);
}

#[test]
fn a_disk_too_small_for_a_table_is_not_read() {
    let got = Contents::read_with(33, |_| panic!("read a disk of 33 sectors"));
    assert_eq!(got, Contents::Unknown);
}
