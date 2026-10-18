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

//! DELIVER's body: every field back as written, the request and response
//! split by their length, and a short, oversized or overrun body refused.

use nonos_proof_abi::wire::{request, NOT_VENDOR, PROVE_MAGIC};
use nonos_proof_abi::wire::{DeliverRequest, DELIVER, DELIVER_DATA_MAX, DELIVER_HEAD, FRAME_MAX};

fn ask<'a>(request: &'a [u8], response: &'a [u8]) -> DeliverRequest<'a> {
    let (l, p, n, o, c) = (&[1; 32], &[2; 256], &[3; 32], &[4; 32], &[5; 32]);
    DeliverRequest {
        listing_id: l,
        policy: p,
        nonce: n,
        order_id: o,
        channel_id: c,
        from: 7,
        to: 9,
        request,
        response,
    }
}

#[test]
fn a_deliver_body_reads_back_as_written() {
    let b = ask(b"ask", b"answer").encode();
    assert_eq!(b.len(), DELIVER_HEAD + 9);
    let d = DeliverRequest::decode(&b).expect("decodes");
    assert_eq!(
        (*d.listing_id, d.policy[0], *d.nonce, *d.order_id, *d.channel_id),
        ([1; 32], 2, [3; 32], [4; 32], [5; 32])
    );
    assert_eq!((d.from, d.to, d.request, d.response), (7, 9, &b"ask"[..], &b"answer"[..]));
    let empty = ask(b"", b"").encode();
    assert_eq!(
        DeliverRequest::decode(&empty).map(|d| (d.request.len(), d.response.len())),
        Some((0, 0))
    );
    assert_eq!(DELIVER, 5);
    assert_eq!(NOT_VENDOR, -1106);
}

#[test]
fn a_short_oversized_or_overrun_body_is_refused() {
    let b = ask(b"ask", b"answer").encode();
    assert!(DeliverRequest::decode(&b[..DELIVER_HEAD - 1]).is_none(), "short");
    let mut over = b.clone();
    over[DELIVER_HEAD - 4..DELIVER_HEAD].copy_from_slice(&10u32.to_le_bytes());
    assert!(DeliverRequest::decode(&over).is_none(), "a request past the body");
    let big = vec![0u8; DELIVER_DATA_MAX + 1];
    assert!(DeliverRequest::decode(&ask(&big, b"").encode()).is_none(), "oversized");
    let most = vec![0u8; DELIVER_DATA_MAX];
    let frame = request(PROVE_MAGIC, DELIVER, 1, &ask(&most, b"").encode());
    assert!(frame.len() <= FRAME_MAX, "the largest DELIVER fits a frame");
}
