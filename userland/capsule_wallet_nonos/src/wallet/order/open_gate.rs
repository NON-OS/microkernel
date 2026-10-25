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

/*
 * When a voucher listing's channel may be opened, and OPEN START's request
 * (B8, v1.4.38): at consent, read from the market as a voucher, with no
 * open channel kept instead. The unspent deposit goes back to `fresh`,
 * the Shield's fresh address, and to nothing the store named.
 */

use nonos_payment_proto::OpenStart;

use super::held::Held;
use super::pending_step::Step;
use super::read::Named;
use super::voucher::Voucher;

/// The voucher listing Open channel acts on, if this is one it may.
pub fn openable<'a>(
    step: &Step,
    read: &'a Result<Named, &'static str>,
    held: &Held,
) -> Option<&'a Voucher> {
    let (Step::Consent, Ok(Named::Voucher(v))) = (step, read) else { return None };
    held.offers_form().then_some(v)
}

/// OPEN START for `amount` until `expiry`, reclaimed to `fresh`; `now` is
/// read only once there is a fresh address.
pub fn open_start(
    v: &Voucher,
    fresh: Result<[u8; 20], &'static str>,
    amount: u128,
    expiry: u64,
    now: impl FnOnce() -> u64,
) -> Result<OpenStart, &'static str> {
    fresh.and_then(|to| v.open(amount, expiry, to, now()))
}
