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

//! The head of a READ or WRITE at every edge, against disks of 2^0 to 2^63
//! units of 512 or 4096 bytes and buffers of 512 bytes to 1 MiB: a run is
//! taken only when it has sectors, fits one buffer, and ends inside the
//! disk with no sum wrapping past it, so a READ's reply is never larger
//! than the buffer it is copied into.

use crate::protocol::{E_INVAL, E_MSGSIZE, RESP_HDR_LEN, STATUS_LEN};
use crate::rw_parse::parse;
use crate::sweep_rng::{rounds, Rng};
use crate::wire_rw_rule::rule;

#[test]
fn a_run_is_taken_only_inside_the_disk_and_one_buffer() {
    let mut seen = [0u32; 4];
    for seed in 1..rounds(300_000) {
        let mut r = Rng::new(seed);
        let unit: u64 = if r.below(2) == 0 { 512 } else { 4096 };
        let buffer = 512u64 << r.below(12);
        let max = (buffer / unit) as u32;
        let capacity = r.next() >> r.below(64);
        let n = r.edge(&[0, 1, max as u64, max as u64 + 1, u32::MAX as u64]) as u32;
        let lba = r.edge(&[
            0,
            capacity.wrapping_sub(n as u64),
            capacity,
            (u64::MAX - n as u64).wrapping_add(1),
            u64::MAX,
        ]);
        let len = r.edge(&[0, 11, 12, 13, 12 + n as u64 * unit]) as usize % 70_000;
        let mut body = vec![0u8; len.max(12)];
        body[..8].copy_from_slice(&lba.to_le_bytes());
        body[8..12].copy_from_slice(&n.to_le_bytes());
        body.truncate(len);
        let got = parse(&body, capacity, max);
        assert_eq!(got, rule(&body, capacity, max), "seed {seed}: lba {lba} n {n} of {capacity}");
        if let Ok((lba, n)) = got {
            assert!(lba + n as u64 <= capacity, "seed {seed}");
            let reply = RESP_HDR_LEN + STATUS_LEN + n as usize * unit as usize;
            assert!(
                reply <= RESP_HDR_LEN + STATUS_LEN + buffer as usize,
                "seed {seed}: past the buffer"
            );
        }
        let at = match got {
            Ok(_) => 0,
            Err(e) if e == E_INVAL => 1,
            Err(e) if e == E_MSGSIZE => 2,
            Err(_) => 3,
        };
        seen[at] += 1;
    }
    assert!(
        cfg!(miri) || seen.iter().all(|&n| n >= 10_000),
        "taken, none, too many, past: {seen:?}"
    );
}
