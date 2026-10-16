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

//! The reading of a named purchase driven call by call against a market
//! whose three answers are given.

use nonos_market_proto::{BondCode, ReleaseEscrow};
use nonos_market_proto::{OP_GET_COMMERCE, OP_GET_RELEASE, OP_INSTALL_READY};
use nonos_marketplace_abi::{commerce_bytes, commerce_from_bytes, SettleMode};
use nonos_payment_proto::Names;

use super::market_answers::{commerce_body, ready_body, release_body};
use super::read_fixture::{commerce, escrow, ready};
use crate::order::read::Named;
use crate::order::reading_step::{answered, call, Read, Reading};

/// One answer for each of the three calls.
pub type Market = [Vec<u8>; 3];
type Calls = Vec<(u16, Vec<u8>)>;

/// An escrow listing sold at `price` per install, passing every gate but
/// the package.
pub fn escrow_market(price: u128) -> Market {
    let mut c = commerce(SettleMode::Escrow);
    let mut block = commerce_from_bytes(&c.block).unwrap();
    block.unit_price = price;
    c.block = commerce_bytes(&block);
    [commerce_body(&c), release_body(&escrow()), ready_body(&ready(&[1], BondCode::Active))]
}

/// A voucher listing, its release with no escrowed package.
pub fn voucher_market() -> Market {
    let bare = ReleaseEscrow { chunk_root: [0; 32], content_len: 0, ..escrow() };
    let ok = ready(&[1], BondCode::Active);
    [commerce_body(&commerce(SettleMode::Voucher)), release_body(&bare), ready_body(&ok)]
}

/// Every call the reading makes against `m`, and what it ends in.
pub fn read_through(names: &Names, m: &Market) -> (Calls, Result<Named, &'static str>) {
    let (mut calls, mut at) = (Vec::new(), Reading::Commerce);
    let ops = [OP_GET_COMMERCE, OP_GET_RELEASE, OP_INSTALL_READY];
    loop {
        let next = call(names, &at).and_then(|(op, body)| {
            calls.push((op, body));
            let answer = &m[ops.iter().position(|o| *o == op).expect("a market op")];
            answered(names.chain_id, at, answer)
        });
        match next {
            Ok(Read::More(n)) => at = n,
            Ok(Read::Done(read)) => return (calls, read),
            Err(why) => return (calls, Err(why)),
        }
    }
}
