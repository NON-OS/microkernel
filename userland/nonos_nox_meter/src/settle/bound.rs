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

//! The bound on another settle() (plan/21, v1.4.7 and v1.4.12). A settle
//! that lied about settled could have the meter sign settle() again and
//! again, each mined one spending gas. So the meter signs another only:
//!
//! - when a named channel has a newer voucher than the last settle carried;
//! - to replace that last one at its own nonce (a fee bump: only one of the
//!   two can be mined);
//! - or, once its account nonce has passed the last settle's (that nonce
//!   went to another transaction, a top-up signed in between), the same
//!   vouchers again, at most once a day. Otherwise they would stay carried
//!   and unpaid until anyone may close; the daily limit bounds what a lying
//!   settle can burn. A `last_retry` more than a day ahead of `now` (the
//!   clock stepped back, or stamped while wrong) is due too, so it cannot
//!   hold a settlement stuck until real time catches up.

use crate::error::MeterError;
use crate::state::MeterState;

/// The meter signs the same vouchers again at most this often, in seconds.
pub const RETRY_EVERY: u64 = 86_400;

/// Whether a settle at `nonce` on `chain_id` may be signed, given whether
/// it carries a newer voucher; the `last_retry` the state then keeps.
pub(super) fn allowed(
    s: &MeterState,
    chain_id: u64,
    nonce: u64,
    newer: bool,
    now: u64,
) -> Result<u64, MeterError> {
    if newer || s.last_settle == Some((chain_id, nonce)) {
        return Ok(s.last_retry);
    }
    let passed = matches!(s.last_settle, Some((c, n)) if c == chain_id && nonce > n);
    if passed && retry_due(s.last_retry, now) {
        return Ok(now);
    }
    Err(MeterError::NothingOwed)
}

/// A day has passed since `last_retry`, or it lies more than a day ahead.
fn retry_due(last_retry: u64, now: u64) -> bool {
    now >= last_retry.saturating_add(RETRY_EVERY) || last_retry.saturating_sub(now) > RETRY_EVERY
}
