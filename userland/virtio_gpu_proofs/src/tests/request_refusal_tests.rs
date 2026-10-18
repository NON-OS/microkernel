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

//! Every frame a client can put in driver.virtio_gpu0's inbox, through the driver's
//! real header decode and, when it is refused, the reply the loop sends: the
//! driver's own encoders under `refused`, with E_INVAL. Each frame either
//! decodes to the request and exact body it carries or draws one whole reply
//! naming the op, flags and request id it carried (zeros when it is too short
//! to carry them), so its caller is not left waiting out its timeout.

use super::frames::{frame, xorshift, BODY_MAX, LAST_OP};
use super::refusal::check;
use crate::protocol::HDR_LEN;

#[test]
fn random_frames_are_served_or_answered_with_a_refusal() {
    let mut s = 0x4E56_4750_0000_0001u64;
    let (mut served, mut refused_n) = (0usize, 0usize);
    for _ in 0..200_000 {
        let r = xorshift(&mut s);
        let body_len = (xorshift(&mut s) % (BODY_MAX.min(512) as u64 + 8)) as usize;
        let body: Vec<u8> = (0..body_len).map(|_| xorshift(&mut s) as u8).collect();
        let len_field = match r % 8 {
            0 => body_len as u32 + 1,
            1 => (body_len as u32).wrapping_sub(1),
            2 => xorshift(&mut s) as u32,
            3 => u32::MAX - (xorshift(&mut s) % 4) as u32,
            _ => body_len as u32,
        };
        let op = (xorshift(&mut s) % (u64::from(LAST_OP) + 2)) as u16;
        let mut f = frame(op, xorshift(&mut s) as u16, xorshift(&mut s) as u32, len_field, &body);
        match (r >> 16) % 16 {
            0 => f[(xorshift(&mut s) % 4) as usize] ^= 1 << (xorshift(&mut s) % 8),
            1 => f[4] ^= 1 << (xorshift(&mut s) % 8),
            2 => f.truncate((xorshift(&mut s) % (HDR_LEN as u64 + 1)) as usize),
            3 => f.iter_mut().for_each(|b| *b = xorshift(&mut s) as u8),
            _ => {}
        }
        if check(&f) {
            served += 1;
        } else {
            refused_n += 1;
        }
    }
    assert!(served > 50_000 && refused_n > 50_000, "both paths reached: {served} {refused_n}");
}
