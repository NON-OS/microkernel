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

//! A market whose record is set field by field, to trace each line of the
//! consent back to it: payee, price and bond in the commerce block, the
//! package's length in the release, the title in GET_APP.

use nonos_marketplace_abi::{commerce_bytes, commerce_from_bytes, SettleMode};
use nonos_payment_proto::Names;

use super::ids_fixture::{commerce_for, KEY};
use super::market_answers::app_body;
use super::market_drive::{market_for, read_terms, Market};
use super::read_fixture::escrow;
use crate::order::buy::Buy;
use crate::order::read::Named;
use crate::order::terms::asked::Record;

/// The EIP-55 example 0x5aAeb6053F3E94C9b9A09f33669435E7Ef1BeAed.
pub const PAYEE: [u8; 20] = [
    0x5a, 0xae, 0xb6, 0x05, 0x3f, 0x3e, 0x94, 0xc9, 0xb9, 0xa0, 0x9f, 0x33, 0x66, 0x94, 0x35, 0xe7,
    0xef, 0x1b, 0xea, 0xed,
];
pub const PAYEE_TEXT: &str = "0x5aAeb6053F3E94C9b9A09f33669435E7Ef1BeAed";
pub const PRICE: u128 = 7_250_000_000_000_000_003;
pub const BOND: [u8; 32] = [0xB0; 32];
pub const LEN: u64 = 123_457;

/// The market's record for `n`: the fields above, titled `title`.
pub fn recorded(n: &Names, title: &[u8]) -> Market {
    let mut c = commerce_for(n, SettleMode::Escrow);
    let mut block = commerce_from_bytes(&c.block).unwrap();
    (block.payee, block.unit_price, block.bond_id) = (PAYEE, PRICE, BOND);
    c.block = commerce_bytes(&block);
    let e = nonos_market_proto::ReleaseEscrow { content_len: LEN, ..escrow() };
    let mut m = market_for(n, &c, &e);
    m[3] = app_body(&n.listing, title, &KEY);
    m
}

/// The escrow terms and the market's names, as the wallet reads `m`.
pub fn read(n: &Names, m: &Market) -> (Buy, Record) {
    match read_terms(n, m).1 {
        Ok(t) => match t.named {
            Named::Escrow(b) => (b, t.record),
            other => panic!("not escrow: {other:?}"),
        },
        Err(why) => panic!("not read: {why}"),
    }
}
