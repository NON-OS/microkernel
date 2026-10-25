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

//! FUNDED (op 21): a chain and a listing string in, one bit out, and no
//! other reading of either.

use nonos_payment_proto::*;

mod common;
use common::refused_cut_and_tail;

fn ask() -> FundedAsk {
    FundedAsk { chain_id: 11_155_111, listing: b"linux.qwen-small".to_vec() }
}

#[test]
fn funded_asks_round_trip_and_carry_no_amount() {
    let b = ask().encode().unwrap();
    assert_eq!(b.len(), 8 + 2 + 16, "chain_id, the listing, and nothing else");
    assert_eq!(FundedAsk::parse(&b), Some(ask()));
    refused_cut_and_tail(&b, FundedAsk::parse);
    let empty = FundedAsk { listing: Vec::new(), ..ask() };
    assert_eq!(empty.encode(), None);
    assert_eq!(FundedAsk::parse(&[&b[..8], &[0, 0][..]].concat()), None, "an empty listing");
    let long = FundedAsk { listing: vec![b'a'; MAX_NAME + 1], ..ask() };
    assert_eq!(long.encode(), None);
    let at_most = FundedAsk { listing: vec![b'a'; MAX_NAME], ..ask() };
    assert_eq!(FundedAsk::parse(&at_most.encode().unwrap()), Some(at_most));
}

#[test]
fn the_answer_is_one_byte_zero_or_one() {
    assert_eq!(parse_funded(&[FUNDED_YES]), Some(true));
    assert_eq!(parse_funded(&[FUNDED_NO]), Some(false));
    for bad in [&[][..], &[2], &[1, 0], &[0, 0], &[0xff]] {
        assert_eq!(parse_funded(bad), None, "{bad:?}");
    }
    assert_eq!((OP_FUNDED, OP_ACCOUNT), (21, 5), "a new op, ACCOUNT untouched");
}
