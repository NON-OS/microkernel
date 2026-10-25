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

//! The scope and context come from the listing, the day and the record, and
//! the container parses as kind 1 on the day, plan/13's sections in order.

use nonos_proof_abi::{ctx, market_scope, parse, tag, Kind};

use crate::fixture::{hist, policy, record, sealed_bytes, LISTING};
use crate::market::container::container;
use crate::market::device::scope_and_context;
use crate::market::isolation::witness;
use crate::market::sealed::split;

const DAY: u64 = 20_372;

#[test]
fn the_container_carries_the_six_sections_in_tag_order() {
    let p = policy(&LISTING, &[0, 1, 60, 231]);
    let (h, meas) = (hist(), vec![[0x11; 32], [0x22; 32]]);
    let s = split(&sealed_bytes(&record(&p, &h, &meas), &h, &meas)).expect("splits");
    let (_, st) = witness(&s, &p).expect("inside");
    let b = container(DAY, &s, &[0xDE; 40], &[0x15; 50], &st, &p).expect("encodes");
    let c = parse(&b).expect("parses");
    assert_eq!((c.kind, c.window), (Kind::Isolation, DAY));
    let words: Vec<u8> = st.words().iter().flat_map(|w| w.to_le_bytes()).collect();
    assert_eq!(c.get(tag::RECORD), Some(&s.raw[..]));
    assert_eq!(c.get(tag::DEVICE), Some(&[0xDE; 40][..]));
    assert_eq!(c.get(tag::ISOLATION), Some(&[0x15; 50][..]));
    assert_eq!(c.get(tag::PUBLICS), Some(&words[..]));
    assert_eq!(c.get(tag::MEASUREMENTS), Some(&meas.concat()[..]));
    assert_eq!(c.get(tag::POLICY), Some(&p[..]));
}

#[test]
fn the_scope_and_context_are_the_listings_and_the_records() {
    let p = policy(&LISTING, &[0, 1, 60, 231]);
    let raw = record(&p, &hist(), &[]).encode();
    let (e, c) = scope_and_context(&LISTING, DAY, &raw);
    assert_eq!(e.map(|f| f.to_u64()), market_scope(&LISTING, DAY));
    assert_eq!(c.map(|f| f.to_u64()), ctx(Kind::Isolation, &raw));
    assert_ne!(scope_and_context(&LISTING, DAY + 1, &raw).0, e, "one scope a day");
    assert_ne!(scope_and_context(&[0x4D; 32], DAY, &raw).0, e, "one scope a listing");
}
