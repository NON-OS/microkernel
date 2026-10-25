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

//! A market's four answers to a named purchase, as market.index would
//! give them for the names asked.

use nonos_market_proto::{BondCode, Commerce, ReleaseEscrow};
use nonos_marketplace_abi::{commerce_bytes, commerce_from_bytes, SettleMode};
use nonos_payment_proto::Names;

use super::gate_fixture::names;
use super::ids_fixture::{commerce_for, version, KEY, TITLE};
use super::market_answers::{app_body, commerce_body, ready_body, release_body};
use super::read_fixture::{escrow, ready};

/// One answer for each of the four calls, in the order they are made.
pub type Market = [Vec<u8>; 4];
/// The market's answers for `n`: `c`, the release `e`, every gate passed
/// but the package, then the listing.
pub fn market_for(n: &Names, c: &Commerce, e: &ReleaseEscrow) -> Market {
    let ok = ready(&[1], BondCode::Active);
    let app = app_body(&n.listing, TITLE, &KEY);
    [commerce_body(c), release_body(e, &version(n)), ready_body(&ok), app]
}

/// An escrow listing named `n`, sold at `price` per install.
pub fn escrow_market_for(n: &Names, price: u128) -> Market {
    let mut c = commerce_for(n, SettleMode::Escrow);
    let mut block = commerce_from_bytes(&c.block).unwrap();
    block.unit_price = price;
    c.block = commerce_bytes(&block);
    market_for(n, &c, &escrow())
}

/// An escrow listing as names() name it, sold at `price` per install.
pub fn escrow_market(price: u128) -> Market {
    escrow_market_for(&names(), price)
}

/// A voucher listing, its release with no escrowed package.
pub fn voucher_market() -> Market {
    let bare = ReleaseEscrow { chunk_root: [0; 32], content_len: 0, ..escrow() };
    market_for(&names(), &commerce_for(&names(), SettleMode::Voucher), &bare)
}
