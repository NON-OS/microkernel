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

//! What a client sends, swept with hostile input: any header bytes, any
//! read or write body against any card size, and a stream of mangled
//! requests at a served card. Nothing panics, every request gets exactly
//! one reply with a status the wire names, and the card stays served.

use nonos_libc::grant_cap;

use super::rig::{sdhc, serving};
use super::rng::Rng;
use super::service_rig::{answer_raw, request, INSTALLER, STORE_WRITE};
use crate::protocol::*;
use crate::serve::medium::Medium;

const NAMED: [i32; 7] = [E_OK, E_INVAL, E_MSGSIZE, E_NXIO, E_ACCES, E_ROFS, E_IO];

#[test]
fn any_header_and_any_range_body_is_decoded_or_refused_inside_its_bounds() {
    let mut rng = Rng::new(0x4D);
    for _ in 0..200_000 {
        let raw = rng.bytes(40);
        if let Some(req) = decode_request(&raw) {
            assert!(raw.len() >= HDR_LEN && raw[..4] == header::MAGIC.to_le_bytes());
            assert!(req.op == u16::from_le_bytes([raw[6], raw[7]]));
        }
        let blocks = rng.edge(&[0, 1, 1 << 20, 1 << 23, 1 << 32]);
        let (lba, count) =
            (rng.edge(&[0, blocks.wrapping_sub(1), blocks, u64::MAX]), rng.edge(&[0, 1, 128, 129]));
        let write = rng.below(2) == 1;
        let data = if write { (count as u32 as usize).min(130) * 512 } else { 0 };
        let mut body =
            [&lba.to_le_bytes()[..], &(count as u32).to_le_bytes(), &vec![0; data]].concat();
        body.truncate(body.len() - rng.below(2) as usize);
        if let Ok(span) = parse_rw(&body, blocks, write) {
            assert!((1..=128).contains(&span.count));
            assert!(span.lba as u64 + span.count as u64 <= blocks);
            assert_eq!(body.len(), 12 + if write { span.count as usize * 512 } else { 0 });
        }
    }
}

#[test]
fn a_stream_of_mangled_requests_gets_one_named_reply_each() {
    let (mut drv, mut medium) = serving(sdhc(1));
    grant_cap(INSTALLER, STORE_WRITE);
    let mut rng = Rng::new(0xF00D);
    for _ in 0..3_000 {
        let op = rng.below(8) as u16;
        let mut rx = request(op, rng.next() as u32, &rng.bytes(40));
        let at = rng.below(rx.len() as u64 + 8) as usize;
        if let Some(b) = rx.get_mut(at) {
            *b ^= rng.next() as u8;
        }
        let sender = [0, INSTALLER, 9][rng.below(3) as usize];
        let reply = answer_raw(&mut drv, &mut medium, sender, &rx);
        assert!(NAMED.contains(&reply.status), "{}", reply.status);
        assert!(matches!(medium, Medium::Ready(_)), "{medium:?}");
    }
}
