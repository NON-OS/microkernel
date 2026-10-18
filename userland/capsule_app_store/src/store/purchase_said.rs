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

//! Which listings the wallet buys, and what the store says once it asked
//! (B8, plan/27, v1.4.40): an escrow listing whose price its publisher and
//! payee both stand behind goes to the wallet by name, through the meter's
//! mailbox; the wallet reads the terms from the market itself.

use nonos_payment_proto::{Names, Ready};

use super::asked::Asked;
use super::listing::Listing;
use super::price::{ESCROW, VOUCHER};

/// Whether Enter on `l` hands its purchase to the wallet: a trusted escrow
/// price, or a trusted voucher price not installed yet, whose channel the
/// wallet opens (v1.4.43).
pub fn by_wallet(l: &Listing) -> bool {
    let voucher = |mode| mode == VOUCHER && l.progress.uninstalled();
    l.known.price.is_some_and(|p| p.trusted && (p.mode == ESCROW || voucher(p.mode)))
}

/// Whether a purchase of `l` becomes an order the store then follows.
pub fn ordered(l: &Listing) -> bool {
    l.known.price.is_some_and(|p| p.mode == ESCROW)
}

/// The meter's answer to 19/0 POST, as the store says it.
pub fn handed(r: Result<(), i32>) -> Asked {
    match r {
        Ok(()) => Asked::InWallet,
        Err(nonos_payment_proto::EBUSY) => Asked::WalletWaiting,
        Err(_) => Asked::NoMeter,
    }
}

/// READY's answer, kept only when its names are the ones this store posted
/// (v1.4.45): a READY left from another purchase never starts an install.
pub fn ours(r: Option<Ready>, posted: Option<&Names>) -> Option<Ready> {
    r.filter(|r| Some(&r.names) == posted)
}
