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

//! What a completion hands on: at most what the device said it wrote, at
//! most what the buffer holds, at most what the caller asked for, and then
//! nothing stays in the buffer (plan/50 C8).

use super::ring::Ring;
use crate::constants::ENTROPY_BUF_LEN;

#[test]
fn a_length_past_the_buffer_is_held_to_the_buffer() {
    let ring = Ring::new();
    let mut q = ring.queue();
    for (n, len) in [4097u32, 0x1_0000, u32::MAX, 0].into_iter().enumerate() {
        q.post().expect("posted");
        ring.complete(n as u16 + 1, 0, len, 0x5A);
        let held = Some(Ok(len.min(ENTROPY_BUF_LEN as u32)));
        assert_eq!(q.completion(), held, "{len}");
        let mut out = vec![0u8; ENTROPY_BUF_LEN + 64];
        let taken = q.take(len, &mut out);
        assert_eq!(taken, len.min(ENTROPY_BUF_LEN as u32) as usize, "{len}");
        assert!(out[taken..].iter().all(|&b| b == 0), "{len}: nothing past the buffer");
    }
}

#[test]
fn the_buffer_is_emptied_once_its_bytes_are_handed_on() {
    let ring = Ring::new();
    let mut q = ring.queue();
    for want in [1usize, 31, 4096] {
        q.post().expect("posted");
        ring.answer(0xC3);
        let len = q.completion().expect("done").expect("ok");
        let mut out = vec![0u8; want];
        assert_eq!(q.take(len, &mut out), want, "the caller asked for {want}");
        assert!(out.iter().all(|&b| b == 0xC3));
        let left = (0..ENTROPY_BUF_LEN).filter(|&at| ring.buf.wrote8(at) != 0).count();
        assert_eq!(left, 0, "{left} entropy bytes stayed in the buffer");
    }
}
