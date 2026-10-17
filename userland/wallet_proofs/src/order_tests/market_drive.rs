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
//! whose four answers are given.

use nonos_market_proto::{OP_GET_APP, OP_GET_COMMERCE, OP_GET_RELEASE, OP_INSTALL_READY};
use nonos_marketplace_abi::Keccak;
use nonos_payment_proto::Names;

use super::deposit_fixture::K;
pub use super::market_build::Market;
pub use super::market_build::{escrow_market, escrow_market_for, market_for, voucher_market};
use crate::order::read::Named;
use crate::order::reading_step::{call, Read, Reading, Terms};
use crate::order::terms::answer::answered;

type Calls = Vec<(u16, Vec<u8>)>;

/// Every call the reading makes against `m`, and the terms it ends in.
pub fn read_terms(names: &Names, m: &Market) -> (Calls, Result<Terms, &'static str>) {
    read_terms_with(&K, names, m)
}

/// The same reading, its ids hashed by `k`.
pub fn read_terms_with(
    k: &impl Keccak,
    names: &Names,
    m: &Market,
) -> (Calls, Result<Terms, &'static str>) {
    let (mut calls, mut at) = (Vec::new(), Reading::Commerce);
    let ops = [OP_GET_COMMERCE, OP_GET_RELEASE, OP_INSTALL_READY, OP_GET_APP];
    loop {
        let next = call(names, &at).and_then(|(op, body)| {
            calls.push((op, body));
            let answer = &m[ops.iter().position(|o| *o == op).expect("a market op")];
            answered(k, names, at, answer)
        });
        match next {
            Ok(Read::More(n)) => at = n,
            Ok(Read::Done(read)) => return (calls, read.map(|t| *t)),
            Err(why) => return (calls, Err(why)),
        }
    }
}

/// The calls, and the terms as read_named judged them.
pub fn read_through(names: &Names, m: &Market) -> (Calls, Result<Named, &'static str>) {
    let (calls, read) = read_terms(names, m);
    (calls, read.map(|t| t.named))
}
