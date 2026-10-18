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

//! HEST's sources walked by their lengths, GHES and v2 read, and BERT.

use super::build::{ghes, header};
use crate::apei::*;

#[test]
fn sources_by_length_and_the_ghes_ones_read() {
    let mut mce = vec![0u8, 0, 1, 0, 0, 0, 0, 1];
    mce.resize(40, 0);
    mce[32] = 2;
    mce.extend(vec![0u8; 56]);
    let mut aer = vec![6u8, 0, 2, 0, 0, 0, 0, 0];
    aer.resize(48, 0);
    let unknown = vec![3u8, 0, 9, 0];
    let body = [
        mce,
        aer,
        ghes(3, 0x7FF0_0000, POLLED, 1000, None),
        ghes(4, 0x7FF0_0100, SCI, 0, Some(0x7FF0_0200)),
        unknown,
    ]
    .concat();
    let t = header(b"HEST", &[6u32.to_le_bytes().to_vec(), body].concat());
    let s = hest(&t);
    let summary: Vec<(u16, u16, bool)> = s.iter().map(|e| (e.kind, e.id, e.enabled)).collect();
    assert_eq!(
        summary,
        [(0, 1, true), (6, 2, false), (9, 3, true), (10, 4, true)],
        "type 3 stops the walk"
    );
    let g = s[2].ghes.unwrap();
    assert_eq!(
        (g.status_address.address, g.notify, g.poll_ms, g.block_len, g.ack),
        (0x7FF0_0000, POLLED, 1000, 0x1000, None)
    );
    let v2 = s[3].ghes.unwrap();
    let ack = v2.ack.unwrap();
    assert_eq!(
        (v2.notify, ack.register.address, ack.preserve, ack.write),
        (SCI, 0x7FF0_0200, !1, 1)
    );
    assert!(s[0].ghes.is_none() && s[1].ghes.is_none());
}
