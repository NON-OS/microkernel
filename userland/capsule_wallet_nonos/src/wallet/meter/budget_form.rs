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
 * The budget form (B8): a cap in whole NOX and how many days it runs, as
 * typed, into the meter's SET_BUDGET. Pure, so wallet_proofs reads it. A
 * new budget starts with nothing spent and replaces the last one whole,
 * per-listing caps included: a listing without its own cap is held only by
 * this one.
 */

use alloc::vec::Vec;

use nonos_payment_proto::SetBudget;

pub const MAX_CAP_NOX: u128 = 1_000_000_000_000;
pub const MAX_DAYS: u64 = 365;
const WEI_PER_NOX: u128 = 1_000_000_000_000_000_000;

pub const NO_CAP: &str = "Type the budget in whole NOX, from 1 to 1,000,000,000,000.";
pub const NO_DAYS: &str = "Type how many days the budget runs, from 1 to 365.";
pub const NO_CLOCK: &str =
    "This machine's clock could not be read, so the budget's end cannot be set.";

fn number(text: &str) -> Option<u128> {
    if text.is_empty() || text.len() > 13 || !text.bytes().all(|b| b.is_ascii_digit()) {
        return None;
    }
    text.parse().ok()
}

/// SET_BUDGET for `cap` whole NOX over `days` days from `now` (unix
/// seconds, 0 when the clock is unread), with no per-listing caps.
pub fn budget_request(cap: &str, days: &str, now: u64) -> Result<SetBudget, &'static str> {
    let cap = number(cap).filter(|c| (1..=MAX_CAP_NOX).contains(c)).ok_or(NO_CAP)?;
    let days = number(days).filter(|d| (1..=MAX_DAYS as u128).contains(d)).ok_or(NO_DAYS)?;
    if now == 0 {
        return Err(NO_CLOCK);
    }
    let expiry = now + days as u64 * 86_400;
    Ok(SetBudget { cap: cap * WEI_PER_NOX, expiry, listings: Vec::new() })
}

/// One typed key into a digits-only field of at most `max` digits; false
/// when it was not taken.
pub fn push_digit(text: &mut alloc::string::String, c: u32, max: usize) -> bool {
    let Some(ch) = char::from_u32(c).filter(char::is_ascii_digit) else {
        return false;
    };
    if text.len() >= max || (text.is_empty() && ch == '0') {
        return false;
    }
    text.push(ch);
    true
}
