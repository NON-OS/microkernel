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

//! Requests as any caller could frame them, from a fixed xorshift seed:
//! every op, short and long payloads, handles that exist and ones that do
//! not, offsets and lengths at every size. ramfs answers each one, and no
//! file it holds is ever past the largest file or the store past its total.

use ramfs_host::store::limits::{MAX_FILE_BYTES, MAX_STORE_BYTES};
use ramfs_host::Ramfs;

use super::wire::{call, frame, open, read_all, CLOSE, OPEN, PID, READ, WRITE};

struct Rng(u64);

impl Rng {
    fn next(&mut self) -> u64 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        self.0
    }

    fn below(&mut self, n: u64) -> u64 {
        self.next() % n.max(1)
    }
}

/// A size near one of the limits, or anything at all.
fn size(rng: &mut Rng) -> u64 {
    let near = [0, 1, MAX_FILE_BYTES as u64, MAX_STORE_BYTES as u64, 1 << 40, u64::MAX];
    match rng.below(3) {
        0 => rng.next(),
        1 => near[rng.below(near.len() as u64) as usize].wrapping_sub(rng.below(3)),
        _ => rng.below(70_000),
    }
}

/// Every file the fuzz can name, through fresh handles, closed again after.
fn sizes(fs: &mut Ramfs) -> Vec<usize> {
    let paths = (0..4).map(|i| format!("/ram/{i}")).chain((0..9).map(|i| format!("/r/{i}")));
    paths
        .map(|path| {
            let h = open(fs, &path);
            let len = read_all(fs, h).len();
            assert_eq!(call(fs, CLOSE, &h.to_le_bytes()).0, 0);
            len
        })
        .collect()
}

#[test]
fn any_request_is_answered_and_no_file_outgrows_the_limits() {
    let mut rng = Rng(0x0BAD_5EED_1234_5678);
    let mut fs = Ramfs::new();
    let mut handles: Vec<u64> = (0..4).map(|i| open(&mut fs, &format!("/ram/{i}"))).collect();
    for round in 0..100_000u32 {
        let op = 1 + rng.below(6) as u16;
        let pick = rng.below(handles.len() as u64) as usize;
        let h = if rng.below(8) == 0 { rng.next() } else { handles[pick] };
        let mut payload = h.to_le_bytes().to_vec();
        payload.extend_from_slice(&size(&mut rng).to_le_bytes());
        match op {
            WRITE => payload.extend(std::iter::repeat_n(round as u8, rng.below(600) as usize)),
            READ => payload.truncate(16 + rng.below(5) as usize),
            _ => payload.truncate(rng.below(20) as usize),
        }
        if op == OPEN && rng.below(2) == 0 {
            payload = 1u32.to_le_bytes().to_vec();
            payload.extend_from_slice(&5u16.to_le_bytes());
            payload.extend_from_slice(format!("/r/{}", rng.below(9)).as_bytes());
        }
        let Some(reply) = fs.serve(&frame(op, &payload), PID) else { continue };
        assert!(reply.len() >= 8, "round {round}: a reply without its header");
        let status = i32::from_le_bytes([reply[4], reply[5], reply[6], reply[7]]);
        match op {
            OPEN if status == 0 => {
                handles.push(u64::from_le_bytes(reply[8..16].try_into().unwrap()))
            }
            CLOSE if status == 0 && h == handles[pick] => _ = handles.swap_remove(pick),
            _ => {}
        }
        if handles.len() > 512 || handles.is_empty() {
            for h in handles.drain(..) {
                call(&mut fs, CLOSE, &h.to_le_bytes());
            }
            handles = (0..4).map(|i| open(&mut fs, &format!("/ram/{i}"))).collect();
        }
        if round % 10_000 == 9_999 {
            let sizes = sizes(&mut fs);
            assert!(sizes.iter().all(|&n| n <= MAX_FILE_BYTES), "round {round}: {sizes:?}");
            assert!(sizes.iter().sum::<usize>() <= MAX_STORE_BYTES, "round {round}: {sizes:?}");
        }
    }
}
