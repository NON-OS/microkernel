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
//! A voucher listing whose channel is open installs on Enter, the wallet's
//! "Use it" answered: the store asks the meter FUNDED (plan/21 op 21), one
//! bit the meter works out from the listing's own price and its channels,
//! and installs on a yes. Anything less, no answer included, posts.

use alloc::vec::Vec;

use nonos_payment_proto::parse_funded;

use super::asked::Asked;
use super::listing::Listing;
use super::price::VOUCHER;
use super::progress::Progress;
use super::purchase_said::by_wallet;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Route {
    /// mk_app_install, as for a listing nobody sells.
    Install,
    /// 19/0 POST to the wallet.
    Post,
}

/// Whether Enter on `l` asks the meter first: a trusted voucher listing the
/// wallet would otherwise be handed. Escrow never does.
pub fn reads_channels(l: &Listing) -> bool {
    by_wallet(l) && l.known.price.is_some_and(|p| p.mode == VOUCHER)
}

/// FUNDED's reply as the store takes it: the bit, or None for an error, no
/// answer, or a reply that does not read.
pub fn answer(reply: Result<Vec<u8>, i32>) -> Option<bool> {
    parse_funded(&reply.ok()?)
}

/// Where Enter sends `l`, given the meter's bit; only Some(true) installs a
/// listing the wallet sells.
pub fn route(l: &Listing, funded: Option<bool>) -> Route {
    match !by_wallet(l) || (reads_channels(l) && funded == Some(true)) {
        true => Route::Install,
        false => Route::Post,
    }
}

/// An install asked for and queued shows as queued.
pub fn queued(l: &mut Listing, asked: Asked) {
    if asked == Asked::Queued {
        (l.progress, l.removing) = (Progress::Queued, false);
    }
}
