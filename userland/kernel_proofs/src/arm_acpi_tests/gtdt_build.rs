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

//! GTDTs of revisions 2 and 3, with GT blocks and watchdogs.

use super::build::*;

fn timers(rev: u8) -> Vec<u8> {
    let mut b = [u64le(0x2A43_0000), u32le(0)].concat();
    for (gsiv, flags) in [(29, 2), (30, 2), (27, 2), (26, 2)] {
        b.extend([u32le(gsiv), u32le(flags)].concat());
    }
    b.extend([u64le(0x2A80_0000), u32le(0), u32le(0)].concat());
    if rev >= 3 {
        b.extend([u32le(28), u32le(4)].concat());
    }
    b
}

pub fn gt_block(frames: u32) -> Vec<u8> {
    let frame = |n: u8| {
        [
            vec![n, 0, 0, 0],
            u64le(0x2A81_0000),
            u64le(0),
            u32le(58),
            u32le(0),
            u32le(59),
            u32le(0),
            u32le(1),
        ]
        .concat()
    };
    let body: Vec<u8> = (0..frames as u8).flat_map(frame).collect();
    let len = 20 + body.len() as u16;
    [vec![0], u16le(len), vec![0], u64le(0x2A81_0000), u32le(frames), u32le(20), body].concat()
}

pub fn watchdog() -> Vec<u8> {
    [vec![1], u16le(28), vec![0], u64le(0x2A44_0000), u64le(0x2A45_0000), u32le(93), u32le(0)]
        .concat()
}

pub fn table(rev: u8, platform: &[Vec<u8>]) -> Vec<u8> {
    let fixed = if rev >= 3 { 104 } else { 96 };
    let mut b = timers(rev);
    b[52..56].copy_from_slice(&u32le(platform.len() as u32));
    b[56..60].copy_from_slice(&u32le(if platform.is_empty() { 0 } else { fixed }));
    sdt(b"GTDT", rev, &[b, platform.concat()].concat())
}
