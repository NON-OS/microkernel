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

//! Every frame a client can put in the compositor's inbox, through the real
//! header decode. Any process holding the endpoint can send any bytes,
//! so the decode must take each one without a panic and either serve it or
//! refuse it with a status and the request the refusal answers. The drain
//! replies to a refusal with exactly what these build, so a refused caller
//! hears back at once instead of waiting out its timeout.

use super::parse_check::*;
use super::parse_frames::*;
use crate::protocol::{parse, HDR_LEN, IPC_PAYLOAD_MAX};

#[test]
fn random_frames_are_served_or_refused_with_a_reply() {
    let mut s = 0x4E43_4D50_0000_0001u64;
    let (mut served, mut refused) = (0usize, 0usize);
    for _ in 0..FUZZ_ROUNDS {
        let r = xorshift(&mut s);
        let body_len = (xorshift(&mut s) % (IPC_PAYLOAD_MAX as u64 + 8)) as usize;
        let body: Vec<u8> = (0..body_len).map(|_| xorshift(&mut s) as u8).collect();
        let len_field = match r % 8 {
            0 => body_len as u32 + 1,
            1 => (body_len as u32).wrapping_sub(1),
            2 => xorshift(&mut s) as u32,
            3 => u32::MAX - (xorshift(&mut s) % 4) as u32,
            _ => body_len as u32,
        };
        let op = match (r >> 8) % 4 {
            0 => xorshift(&mut s) as u16,
            _ => (xorshift(&mut s) % 0x10) as u16,
        };
        let mut f = frame(op, xorshift(&mut s) as u16, xorshift(&mut s) as u32, len_field, &body);
        match (r >> 16) % 16 {
            0 => f[(xorshift(&mut s) % 4) as usize] ^= 1 << (xorshift(&mut s) % 8),
            1 => f[4] ^= 1 << (xorshift(&mut s) % 8),
            2 => f.truncate((xorshift(&mut s) % (HDR_LEN as u64 + 1)) as usize),
            3 => {
                for b in f.iter_mut() {
                    *b = xorshift(&mut s) as u8;
                }
            }
            _ => {}
        }
        if parse(&f).is_ok() {
            served += 1;
        } else {
            refused += 1;
        }
        check(&f);
    }
    assert!(served > FUZZ_ROUNDS / 4, "the generator reaches the dispatch: {served}");
    assert!(refused > FUZZ_ROUNDS / 4, "the generator reaches the refusals: {refused}");
}
