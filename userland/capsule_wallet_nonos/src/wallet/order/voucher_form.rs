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
 * A voucher channel's deposit and how long it runs, as typed on the
 * purchase screen (B8, v1.4.38): whole NOX, and days from 1 to 365.
 */

use alloc::string::String;

pub const FIELD_DEPOSIT: u8 = 0;
pub const FIELD_DAYS: u8 = 1;

const WEI_PER_NOX: u128 = 1_000_000_000_000_000_000;
const NO_DEPOSIT: &str = "Type the deposit in whole NOX, from 1 to 1,000,000,000,000.";
const NO_DAYS: &str = "Type how many days the channel runs, from 1 to 365.";
const NO_CLOCK: &str =
    "This machine's clock could not be read, so the channel's end cannot be set.";

#[derive(Default)]
pub struct VoucherForm {
    pub deposit: String,
    pub days: String,
    pub focus: u8,
}

fn number(text: &str, most: u128) -> Option<u128> {
    let n: u128 = text.parse().ok().filter(|_| text.bytes().all(|b| b.is_ascii_digit()))?;
    (1..=most).contains(&n).then_some(n)
}

impl VoucherForm {
    /// The deposit in wei and the expiry in unix seconds, from `now`.
    pub fn read(&self, now: u64) -> Result<(u128, u64), &'static str> {
        let deposit = number(&self.deposit, 1_000_000_000_000).ok_or(NO_DEPOSIT)?;
        let days = number(&self.days, 365).ok_or(NO_DAYS)? as u64;
        if now == 0 {
            return Err(NO_CLOCK);
        }
        Ok((deposit * WEI_PER_NOX, now + days * 86_400))
    }
}
