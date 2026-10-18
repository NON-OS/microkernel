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

//! The reply decoder over replies no driver should send: any received
//! count, any declared payload length, any bytes. It never panics, and what
//! it hands back lies inside what was received.

use super::decode_tests::reply;
use crate::wire::{decode_reply, HDR_LEN, STATUS_LEN};

fn next(s: &mut u64) -> u64 {
    *s ^= *s << 13;
    *s ^= *s >> 7;
    *s ^= *s << 17;
    *s
}

#[test]
fn any_reply_decodes_or_is_refused_and_never_reaches_past_what_was_received() {
    for seed in 1..60_000u64 {
        let mut s = seed;
        let mut r = reply(0x4E42_4C4B, 3, 7, (next(&mut s) % 3) as i32 - 1, &[0xA5; 40]);
        let len = (next(&mut s) % 72) as usize;
        r.resize(len, 0x5A);
        if seed % 2 == 0 && r.len() >= HDR_LEN {
            let declared = match seed % 6 {
                0 => next(&mut s) as u32,
                2 => u32::MAX - (next(&mut s) % 8) as u32,
                _ => (next(&mut s) % 48) as u32,
            };
            r[16..20].copy_from_slice(&declared.to_le_bytes());
        }
        let n = (next(&mut s) % 80) as usize;
        if let Ok(payload) = decode_reply(&r, n, 0x4E42_4C4B, 3, 7) {
            let declared = u32::from_le_bytes([r[16], r[17], r[18], r[19]]) as usize;
            assert!(n <= r.len() && HDR_LEN + declared <= n, "seed {seed}");
            assert_eq!(payload.len(), declared - STATUS_LEN, "seed {seed}");
            assert_eq!(payload, &r[HDR_LEN + STATUS_LEN..HDR_LEN + declared]);
        }
    }
}

#[test]
fn a_declared_payload_past_the_reply_or_short_of_a_status_is_refused() {
    let r = reply(0x4E42_4C4B, 3, 7, 0, &[1; 16]);
    for declared in [0u32, 1, 3, 21, 1 << 20, u32::MAX] {
        let mut bad = r.clone();
        bad[16..20].copy_from_slice(&declared.to_le_bytes());
        let got = decode_reply(&bad, bad.len(), 0x4E42_4C4B, 3, 7);
        assert_eq!(got, Err(crate::error::BlkError::BadLength), "declared {declared}");
    }
}
