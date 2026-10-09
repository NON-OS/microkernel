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

//! A hundred thousand random and damaged frames through the real ingress.

use crate::link::{arrive, echo_request, frame, fresh, poll, sent, LOCAL, REMOTE};

fn xorshift(state: &mut u32) -> u32 {
    *state ^= *state << 13;
    *state ^= *state >> 17;
    *state ^= *state << 5;
    *state
}

/// No frame panics the ingress, nothing delivered is larger than the frame
/// it came in, and every frame sent back is an echo reply to us.
#[test]
fn random_and_damaged_frames_never_panic_or_leak() {
    let _g = fresh();
    let mut s = 0x1F00_D00Du32;
    for round in 0..100_000u32 {
        let mut f = match round % 3 {
            0 => (0..xorshift(&mut s) % 120).map(|_| xorshift(&mut s) as u8).collect(),
            1 => frame(REMOTE, LOCAL, 1, &echo_request(xorshift(&mut s) as u16, 1, b"abcd")),
            _ => {
                let len = (xorshift(&mut s) % 64) as usize;
                let data: Vec<u8> = (0..len).map(|_| xorshift(&mut s) as u8).collect();
                frame(REMOTE, LOCAL, xorshift(&mut s) as u8, &data)
            }
        };
        if round % 3 != 0 && !f.is_empty() {
            for _ in 0..xorshift(&mut s) % 3 {
                let at = xorshift(&mut s) as usize % f.len();
                f[at] = xorshift(&mut s) as u8;
            }
        }
        let limit = f.len();
        arrive(f);
        let (_, got) = poll((xorshift(&mut s) % 3) as u8 * 8 + 1);
        if let Some((_, _, payload)) = got {
            assert!(payload.len() <= limit);
        }
        for out in sent() {
            assert!(out.len() >= 42 && out[14 + 9] == 1 && out[34] == 0, "only echo replies go out");
            assert_eq!(&out[14 + 12..14 + 16], &LOCAL[..]);
        }
    }
}
