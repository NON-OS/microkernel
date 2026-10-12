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

//! Hostile bytes for every parser on the wire (the owner's rule 8): none may
//! panic, and what parses encodes back to the same bytes.

use nonos_payment_proto::*;

/// A fixed xorshift, so a failure names its seed and repeats.
fn bytes(seed: u64, len: usize) -> Vec<u8> {
    let mut x = seed.wrapping_mul(0x9E37_79B9_7F4A_7C15) | 1;
    (0..len)
        .map(|_| {
            x ^= x << 13;
            x ^= x >> 7;
            x ^= x << 17;
            x as u8
        })
        .collect()
}

/// Inputs of every length to 300, then a few long ones, each in three fills.
fn hostile() -> impl Iterator<Item = Vec<u8>> {
    let lens = (0..300).chain([511, 512, 1024, 4096, 70_000]);
    lens.flat_map(|n| [bytes(n as u64 + 1, n), vec![0; n], vec![0xff; n]])
}

#[test]
fn no_parser_panics_on_hostile_bytes() {
    for b in hostile() {
        let _ = Account::parse(&b);
        let _ = SetBudget::parse(&b);
        let _ = ListingCapAsk::parse(&b);
        let _ = (TopUp::parse(&b), Close::parse(&b), Drain::parse(&b));
        let _ = (VoucherRecord::parse(&b), DrainRequest::parse(&b));
        let _ = (parse_header(&b), parse_job_id(&b), parse_open(&b));
        let _ = (OfferAsk::parse(&b), OfferMade::parse(&b), OrderNew::parse(&b));
        let _ = (DeliveredAsk::parse(&b), DeliveredMade::parse(&b));
        let _ = (OrderMade::parse(&b), OrderStatus::parse(&b), parse_refund_reply(&b));
        let _ = (PayRequest::parse(&b), PlainPage::parse(&b), PlainAsk::parse(&b));
        let _ = (Names::parse(&b), Ready::parse(&b), parse_purchase(&b));
        let _ = reply_body(&b, 1, 1);
    }
}

#[test]
fn what_parses_encodes_back_to_the_same_bytes() {
    for b in hostile() {
        if let Some(s) = OrderStatus::parse(&b) {
            assert_eq!(s.encode(), b, "OrderStatus");
        }
        if let Some(m) = OrderMade::parse(&b) {
            assert_eq!(m.encode(), b, "OrderMade");
        }
        if let Some(d) = DeliveredMade::parse(&b) {
            assert_eq!(d.encode(), b, "DeliveredMade");
        }
        if let Some(a) = Account::parse(&b) {
            assert_eq!(a.encode(), Some(b.clone()), "Account");
        }
    }
}
