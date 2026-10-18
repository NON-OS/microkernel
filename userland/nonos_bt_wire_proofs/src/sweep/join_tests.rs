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
//! Joining ACL fragments into L2CAP PDUs (Vol 3, Part A, 7.2): a PDU cut
//! into any fragments comes back whole at its last; fragments that
//! overrun, a continuation with no start, or a PDU past the bound is
//! refused and nothing of it is held; a start with no end is counted, and
//! the start after it kept.

use super::rng::{each, Rng};
use nonos_bt_wire::hci::Boundary;
use nonos_bt_wire::l2cap::{basic, Reassembler};
use nonos_bt_wire::WireError;

const MAX: usize = 512;

/// A PDU of up to `MAX` octets, cut into fragments: the first a start.
fn cut(rng: &mut Rng, pdu: &[u8]) -> Vec<(Boundary, Vec<u8>)> {
    let mut out = Vec::new();
    let mut at = 0;
    while at < pdu.len() {
        let n = (1 + rng.below(40)).min(pdu.len() - at);
        let b = if at == 0 { rng.pick(&[0u16, 2]) } else { 1 };
        out.push((Boundary::from_bits(b), pdu[at..at + n].to_vec()));
        at += n;
    }
    out
}

#[test]
fn fragments_join_and_every_lie_leaves_nothing_held() {
    each(|rng| {
        let mut j = Reassembler::new(MAX);
        let pdu = basic(rng.next() as u16, &rng.span(0, MAX - 3)).unwrap();
        let parts = cut(rng, &pdu);
        let (last, head) = parts.split_last().unwrap();
        head.iter().for_each(|(b, d)| assert_eq!(j.push(*b, d), Ok(None)));
        assert_eq!(j.push(last.0, &last.1), Ok(Some(pdu.clone())), "whole at its last");
        let extra = rng.span(1, 8);
        match rng.below(4) {
            0 => assert_eq!(j.push(Boundary::Continuing, &extra), Err(WireError::Sequence)),
            1 => {
                head.iter().for_each(|(b, d)| drop(j.push(*b, d)));
                let over = j.push(last.0, &[&last.1[..], &extra].concat());
                assert_eq!(over, Err(WireError::Length), "fragments past the stated length");
            }
            2 => {
                let len = ((MAX - 3 + rng.below(1000)) as u16).to_le_bytes();
                assert_eq!(j.push(Boundary::FirstNonFlushable, &len), Err(WireError::TooLong));
            }
            _ => {
                let lost = j.dropped();
                head.iter().for_each(|(b, d)| drop(j.push(*b, d)));
                let out = cut(rng, &pdu).iter().map(|(b, d)| j.push(*b, d)).last().unwrap();
                assert_eq!(j.dropped(), lost + u32::from(!head.is_empty()), "a start with no end");
                assert_eq!(out, Ok(Some(pdu.clone())), "and the next start is kept");
            }
        }
        let fresh = basic(1, &[7]).unwrap();
        assert_eq!(j.push(Boundary::FirstNonFlushable, &fresh), Ok(Some(fresh)), "nothing held");
    });
}
