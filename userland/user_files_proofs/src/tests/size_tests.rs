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

//! Large files streamed in bounded memory, holes, the quota, and a full
//! volume.

use nonos_user_files::errno::{EDQUOT, EFBIG, ENOSPC};
use nonos_user_files::limits::{CHUNK, DIRTY_CHUNKS, MAX_FILE};
use nonos_user_files::wire::flags::{CREATE, READ, WRITE};
use nonos_user_files::Broker;

use super::{fresh, get_file, pattern, settle, APP};
use crate::doubles::{client, Desk, Volume};

#[test]
fn a_large_file_is_written_out_as_it_comes() {
    let (mut b, desk) = fresh();
    let body = pattern(1, 40 << 20);
    let (h, _) =
        client(&mut b, &desk, APP).open(b"Downloads/big.iso", WRITE | CREATE, 0o644).unwrap();
    for (i, piece) in body.chunks(256 << 10).enumerate() {
        client(&mut b, &desk, APP).write(h, (i * (256 << 10)) as u64, piece).unwrap();
        // What is held in RAM never passes the dirty budget.
        let written = b.objects().put_bytes;
        let held = (i + 1) * (256 << 10) - written.min((i + 1) * (256 << 10));
        assert!(held <= DIRTY_CHUNKS * CHUNK, "{held} bytes held after {i} writes");
    }
    client(&mut b, &desk, APP).close(h).unwrap();
    settle(&mut b, &desk);
    assert_eq!(get_file(&mut b, &desk, b"Downloads/big.iso").unwrap(), body);
}

#[test]
fn a_hole_reads_as_zeros_and_takes_no_object() {
    let (mut b, desk) = fresh();
    let mut c = client(&mut b, &desk, APP);
    let (h, _) = c.open(b"Documents/sparse", READ | WRITE | CREATE, 0o644).unwrap();
    c.write(h, 10 * CHUNK as u64 + 5, b"tail").unwrap();
    c.fsync(h).unwrap();
    let mut buf = vec![1u8; 4096];
    assert_eq!(c.read(h, 3 * CHUNK as u64, &mut buf).unwrap(), 4096);
    assert!(buf.iter().all(|b| *b == 0));
    drop(c);
    // One chunk, the body and the slot.
    assert!(b.objects().objects.len() <= 4, "{} objects", b.objects().objects.len());
}

#[test]
fn bytes_cut_off_never_come_back_on_a_grow() {
    let (mut b, desk) = fresh();
    let mut c = client(&mut b, &desk, APP);
    let (h, _) = c.open(b"Documents/t", READ | WRITE | CREATE, 0o644).unwrap();
    for (i, piece) in pattern(9, 3 * CHUNK).chunks(256 << 10).enumerate() {
        c.write(h, (i * (256 << 10)) as u64, piece).unwrap();
    }
    c.fsync(h).unwrap();
    c.ftruncate(h, CHUNK as u64 + 10).unwrap();
    c.fsync(h).unwrap();
    c.ftruncate(h, 3 * CHUNK as u64).unwrap();
    let mut buf = vec![1u8; 64];
    assert_eq!(c.read(h, CHUNK as u64 + 10, &mut buf).unwrap(), 64);
    assert!(buf.iter().all(|b| *b == 0));
    c.read(h, CHUNK as u64, &mut buf[..10]).unwrap();
    assert_eq!(&buf[..10], &pattern(9, 3 * CHUNK)[CHUNK..CHUNK + 10]);
}

#[test]
fn the_quota_is_edquot_and_the_volume_full_is_enospc() {
    let desk = Desk::new(&[APP]);
    let mut b = Broker::new(Volume::default(), 3 * CHUNK as u64);
    let mut c = client(&mut b, &desk, APP);
    let (h, _) = c.open(b"Documents/q", READ | WRITE | CREATE, 0o644).unwrap();
    for i in 0..12 {
        c.write(h, (i * (256 << 10)) as u64, &[7u8; 256 << 10]).unwrap();
    }
    assert_eq!(c.write(h, 3 * CHUNK as u64, b"x").map(drop), Err(EDQUOT));
    assert_eq!(c.ftruncate(h, 4 * CHUNK as u64), Err(EDQUOT));
    assert_eq!(c.write(h, MAX_FILE, b"x").map(drop), Err(EFBIG));
    c.ftruncate(h, 0).unwrap();
    c.write(h, 0, b"room again").unwrap();
    drop(c);
    b.objects().full = true;
    let mut c = client(&mut b, &desk, APP);
    assert_eq!(c.fsync(h), Err(ENOSPC));
    drop(c);
    b.objects().full = false;
    client(&mut b, &desk, APP).fsync(h).unwrap();
    let _ = settle;
}
