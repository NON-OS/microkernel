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

//! The pins the catalog holds each wallpaper to, against the real files:
//! every wallpaper in the collection, packed in catalog order, is served
//! whole and verifies; a byte changed on the device is refused rather than
//! shown; the one wallpaper held is let go after its last chunk; and an
//! installed disk that kept only some serves those and no other.

use nonos_libc::take_replies;

use crate::catalog::{count, get_size, get_slug, with_bytes, Fetch};
use crate::collection::{files, flip, installed, reset, TURN};
use crate::protocol::{Header, CHUNK_MAX, E_IO, E_OK, HDR_LEN, OP_GET_CHUNK};
use crate::server::serve::serve;

fn chunk(index: u32, offset: u32) -> (u16, Vec<u8>) {
    let mut f = vec![0u8; HDR_LEN];
    Header { op: OP_GET_CHUNK, status: 0, index, offset, payload_len: 0 }.encode(&mut f);
    let _ = take_replies();
    serve(23, &f);
    let (_, reply) = take_replies().remove(0);
    let hdr = Header::decode(&reply).unwrap();
    (hdr.status, reply[HDR_LEN..].to_vec())
}

#[test]
fn every_wallpaper_is_served_whole_and_matches_its_file() {
    let _turn = TURN.lock().unwrap_or_else(|e| e.into_inner());
    reset();
    let files = files();
    assert_eq!(count() as usize, files.len());
    for (i, (slug, bytes)) in files.iter().enumerate() {
        let i = i as u32;
        assert_eq!(get_slug(i), Some(slug.as_bytes()));
        assert_eq!(get_size(i), Some(bytes.len() as u32));
        let mut got = Vec::new();
        while got.len() < bytes.len() {
            let (status, body) = chunk(i, got.len() as u32);
            assert_eq!(status, E_OK, "{slug} at {}", got.len());
            assert!(!body.is_empty() && body.len() <= CHUNK_MAX);
            got.extend_from_slice(&body);
        }
        assert!(got == *bytes, "{slug} came back whole");
    }
}

#[test]
fn a_changed_byte_on_the_device_is_refused_not_shown() {
    let _turn = TURN.lock().unwrap_or_else(|e| e.into_inner());
    reset();
    let files = files();
    // A byte inside the third wallpaper: it, and only it, is refused.
    let at = files[0].1.len() + files[1].1.len() + 1000;
    flip(at);
    assert_eq!(chunk(2, 0).0, E_IO);
    assert_eq!(with_bytes(2, |_| ()), Err(Fetch::Unavailable));
    assert_eq!(chunk(1, 0).0, E_OK);
    assert_eq!(chunk(3, 0).0, E_OK);
    reset();
    assert_eq!(chunk(2, 0).0, E_OK);
}

#[test]
fn the_held_wallpaper_is_let_go_after_its_last_chunk() {
    let _turn = TURN.lock().unwrap_or_else(|e| e.into_inner());
    reset();
    let len = get_size(0).unwrap();
    assert_eq!(chunk(0, 0).0, E_OK);
    // Changed now, the held copy still answers: nothing is read again mid-way.
    flip(10);
    assert_eq!(chunk(0, CHUNK_MAX as u32).0, E_OK);
    let last = (len - 1) / CHUNK_MAX as u32 * CHUNK_MAX as u32;
    assert_eq!(chunk(0, last).0, E_OK);
    // The last chunk let it go, so the next ask reads the device, and fails.
    assert_eq!(chunk(0, 0).0, E_IO);
    reset();
}

#[test]
fn an_installed_disk_serves_the_kept_ones_alone_and_no_other() {
    let _turn = TURN.lock().unwrap_or_else(|e| e.into_inner());
    installed(&[3, 40, 62]);
    for i in [3, 40, 62] {
        let (status, body) = chunk(i, 0);
        assert_eq!(status, E_OK, "kept {i}");
        assert_eq!(&body[..], &files()[i as usize].1[..body.len()]);
    }
    for i in [0, 5, 55] {
        assert_eq!(chunk(i, 0).0, E_IO, "not kept {i}");
    }
    reset();
}
