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
//! Association and reassociation responses and SA Query action bodies:
//! built sound they read as built; another subtype, another category or
//! action, or a frame cut short is not one; any bytes at all are read
//! without a panic.

use super::rng::{each, Rng};
use nonos_wifi_core::dot11::parse::{parse_assoc_response, parse_reassoc_response};
use nonos_wifi_core::pmf::{parse_sa_query, SaQueryAction};

/// A management frame of `subtype`, its header random, then `body`.
pub fn mgmt(rng: &mut Rng, kind: u8, subtype: u8, body: &[u8]) -> Vec<u8> {
    let fc = [kind << 2 | subtype << 4, rng.byte()];
    [&fc[..], &rng.bytes(22), body].concat()
}

#[test]
fn responses_read_as_built_and_nothing_else_is_one() {
    type Parse = fn(&[u8]) -> Option<(u16, u16)>;
    let both: [(u8, Parse); 2] = [(1, parse_assoc_response), (3, parse_reassoc_response)];
    each(|rng| {
        let (status, aid) = (rng.next() as u16, rng.next() as u16);
        let tail = rng.span(0, 60);
        let body = [&rng.bytes(2)[..], &status.to_le_bytes(), &aid.to_le_bytes(), &tail].concat();
        for (subtype, parse) in both {
            let f = mgmt(rng, 0, subtype, &body);
            assert_eq!(parse(&f), Some((status, aid & 0x3fff)));
            let other = rng.below(16) as u8;
            let kind = 1 + rng.below(3) as u8;
            if other != subtype {
                assert_eq!(parse(&mgmt(rng, 0, other, &body)), None, "subtype {other}");
            }
            assert_eq!(parse(&mgmt(rng, kind, subtype, &body)), None, "frame type {kind}");
            assert_eq!(parse(&f[..rng.below(30)]), None, "cut short");
            let junk = rng.span(0, 80);
            let _ = parse(&junk);
        }
    });
}

#[test]
fn sa_query_bodies_read_as_built_and_nothing_else_is_one() {
    each(|rng| {
        let (action, txid) = (rng.below(2) as u8, [rng.byte(), rng.byte()]);
        let tail = rng.span(0, 20);
        let body = [&[8, action, txid[0], txid[1]][..], &tail].concat();
        let want = if action == 0 { SaQueryAction::Request } else { SaQueryAction::Response };
        assert_eq!(parse_sa_query(&body), Some((want, txid)));
        let mut lie = body.clone();
        lie[0] = 9 + rng.below(247) as u8;
        assert_eq!(parse_sa_query(&lie), None, "category {}", lie[0]);
        let mut lie = body.clone();
        lie[1] = 2 + rng.below(254) as u8;
        assert_eq!(parse_sa_query(&lie), None, "action {}", lie[1]);
        assert_eq!(parse_sa_query(&body[..rng.below(4)]), None, "cut short");
    });
}
