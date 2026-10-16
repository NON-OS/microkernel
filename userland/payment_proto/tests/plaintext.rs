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

//! ORDER_PLAINTEXT's wire (plan/29): the ask exactly, and pages that cover
//! the plaintext, stay inside it, and read back; reads repeat.

use nonos_payment_proto::{PlainAsk, PlainPage, PLAINTEXT_PAGE, PLAIN_ASK_LEN};

#[test]
fn an_ask_is_the_order_and_an_offset() {
    let a = PlainAsk { order_id: [1; 32], offset: 61_440 };
    let b = a.encode();
    assert_eq!((b.len(), &b[32..]), (PLAIN_ASK_LEN, &61_440u64.to_le_bytes()[..]));
    assert_eq!(PlainAsk::parse(&b), Some(a));
    assert_eq!(PlainAsk::parse(&b[..39]), None);
}

#[test]
fn pages_cover_the_plaintext_and_stay_inside_it() {
    let plain: Vec<u8> = (0..150_000u32).map(|i| i as u8).collect();
    let (mut at, mut got) = (0u64, Vec::new());
    while (at as usize) < plain.len() {
        let p = PlainPage::parse(&PlainPage::of(&plain, at).unwrap().encode()).unwrap();
        assert!(p.bytes.len() <= PLAINTEXT_PAGE && p.total == 150_000 && p.offset == at);
        at += p.bytes.len() as u64;
        got.extend_from_slice(&p.bytes);
    }
    assert_eq!(got, plain);
    assert_eq!(PlainPage::of(&plain, 0), PlainPage::of(&plain, 0), "reads repeat");
    assert_eq!(PlainPage::of(&plain, 150_000), None, "past the end");
    let lying = PlainPage { total: 10, offset: 8, bytes: vec![1; 3] }.encode();
    assert_eq!(PlainPage::parse(&lying), None, "past its total");
    let long =
        PlainPage { total: u64::MAX, offset: 0, bytes: vec![0; PLAINTEXT_PAGE + 1] }.encode();
    assert_eq!(PlainPage::parse(&long), None, "longer than a page");
}
