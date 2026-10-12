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

//! DRAIN's two forms, TOPUP and CLOSE.

use nonos_payment_proto::*;

fn sign(n: usize) -> DrainSign {
    DrainSign {
        chain_id: 11_155_111,
        nonce: 4,
        gas_limit: 300_000,
        max_fee: 9,
        max_priority: 2,
        channels: (0..n).map(|i| [i as u8 + 1; 32]).collect(),
    }
}

#[test]
fn list_and_sign_are_told_apart_by_their_first_byte() {
    let list = DrainRequest::List { from: 12 };
    assert_eq!(list.encode().unwrap(), vec![DRAIN_LIST, 12]);
    assert_eq!(DrainRequest::parse(&list.encode().unwrap()), Some(list));
    let s = DrainRequest::Sign(sign(3));
    let b = s.encode().unwrap();
    assert_eq!((b[0], b.len()), (DRAIN_SIGN, 1 + 24 + 32 + 1 + 3 * 32));
    assert_eq!(DrainRequest::parse(&b), Some(s));
    for n in 0..b.len() {
        assert_eq!(DrainRequest::parse(&b[..n]), None, "{n}");
    }
    assert_eq!(DrainRequest::parse(&[b.as_slice(), &[0]].concat()), None, "a tail");
    assert_eq!(DrainRequest::parse(&[2, 0]), None, "no third form");
}

#[test]
fn sign_names_one_to_twelve_channels_none_twice() {
    assert_eq!(DrainRequest::parse(&DrainRequest::Sign(sign(0)).encode().unwrap()), None, "none");
    let most = DrainRequest::Sign(sign(DRAIN_ROWS_MAX));
    assert!(DrainRequest::parse(&most.encode().unwrap()).is_some());
    let past = DrainRequest::Sign(sign(DRAIN_ROWS_MAX + 1));
    assert_eq!(past.encode(), None, "one past DRAIN_ROWS_MAX is refused, not written");
    let mut twice = sign(2);
    twice.channels[1] = twice.channels[0];
    assert_eq!(DrainRequest::parse(&DrainRequest::Sign(twice).encode().unwrap()), None, "twice");
}

#[test]
fn topup_and_close_round_trip() {
    let t = TopUp { chain_id: 1, channel_id: [5; 32], amount: 7 };
    let b = t.encode();
    assert_eq!((b.len(), TopUp::parse(&b)), (56, Some(t)));
    let c = Close { chain_id: 1, channel_id: [5; 32] };
    assert_eq!((c.encode().len(), Close::parse(&c.encode())), (40, Some(c)));
    assert_eq!(Close::parse(&b), None, "a topup is no close");
    assert_eq!(TopUp::parse(&c.encode()), None);
    assert_eq!((OP_TOPUP, OP_CLOSE), (13, 14));
}
