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

/* A voucher channel's deposit, days and listing cap as typed (B8, v1.4.38,
 * v1.4.43): whole NOX, 1 to 365 days, and the cap the deposit unless typed. */

use alloc::string::String;

pub const FIELD_DEPOSIT: u8 = 0;
pub const FIELD_DAYS: u8 = 1;
pub const FIELD_CAP: u8 = 2;

const WEI_PER_NOX: u128 = 1_000_000_000_000_000_000;
const NO_DEPOSIT: &str = "Type the deposit in whole NOX, from 1 to 1,000,000,000,000.";
const NO_CAP: &str = "Type the listing's cap in whole NOX, or leave it for the deposit.";
const NO_DAYS: &str = "Type how many days the channel runs, from 1 to 365.";
const NO_CLOCK: &str = "The clock could not be read, so the channel's end cannot be set.";

/// Where the listing's cap stands once the channel is open (op 20).
#[derive(Clone, Copy, Default, PartialEq, Eq, Debug)]
pub enum Budgeted {
    #[default]
    Unasked,
    Set,
    /// The global budget is zero or past its end: PAY would refuse.
    NoBudget,
    Failed(&'static str),
}

#[derive(Default)]
pub struct VoucherForm {
    pub deposit: String,
    pub days: String,
    pub cap: String,
    pub focus: u8,
    /// The cap in wei, as read when the channel was asked for.
    pub cap_wei: u128,
    pub budgeted: Budgeted,
}

fn number(text: &str, most: u128) -> Option<u128> {
    let n: u128 = text.parse().ok().filter(|_| text.bytes().all(|b| b.is_ascii_digit()))?;
    (1..=most).contains(&n).then_some(n)
}

impl VoucherForm {
    /// The deposit and the cap in wei, and the expiry in unix seconds.
    pub fn read(&self, now: u64) -> Result<(u128, u64, u128), &'static str> {
        let deposit = number(&self.deposit, 1_000_000_000_000).ok_or(NO_DEPOSIT)?;
        let days = number(&self.days, 365).ok_or(NO_DAYS)? as u64;
        let cap = match self.cap.is_empty() {
            true => deposit,
            false => number(&self.cap, 1_000_000_000_000).ok_or(NO_CAP)?,
        };
        if now == 0 {
            return Err(NO_CLOCK);
        }
        Ok((deposit * WEI_PER_NOX, now + days * 86_400, cap * WEI_PER_NOX))
    }
}
