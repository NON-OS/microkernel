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

//! Where a purchase's funding quote stands, and an amount written exactly.

use alloc::format;
use alloc::string::String;

pub enum Fees {
    Unasked,
    Asking,
    /// The service's total fee, as it wrote it: whole NOX and decimals.
    Quoted(String),
    Refused(String),
}

/// `wei` as NOX: the whole part, then every significant decimal.
pub fn exact(wei: u128) -> String {
    let unit = 1_000_000_000_000_000_000u128;
    let frac = format!("{:018}", wei % unit);
    match frac.trim_end_matches('0') {
        "" => format!("{}", wei / unit),
        f => format!("{}.{f}", wei / unit),
    }
}

/// Said instead of a fee the Shield wrote that does not read as NOX.
pub const FEE_UNREAD: &str = "The Shield's fee for this price does not read as an amount of \
     NOX, so the wallet will not pay it.";

/// The Shield's answer to the quote: its refusal, or its total fee, which
/// must read as NOX (fee_wei) to be quoted at all.
pub fn quote_answer(refusal: Option<&str>, total_fee: Option<&str>) -> Fees {
    match (refusal, total_fee) {
        (Some(why), _) => Fees::Refused(why.into()),
        (None, Some(fee)) if fee_wei(fee).is_some() => Fees::Quoted(fee.into()),
        (None, _) => Fees::Refused(FEE_UNREAD.into()),
    }
}

/// `text` as wei of NOX: whole NOX, then at most 18 decimals; nothing else.
pub fn fee_wei(text: &str) -> Option<u128> {
    let (whole, frac) = text.split_once('.').unwrap_or((text, ""));
    let digits = |s: &str| s.bytes().all(|b| b.is_ascii_digit());
    if whole.is_empty() || !digits(whole) || !digits(frac) || frac.len() > 18 {
        return None;
    }
    if text.ends_with('.') {
        return None;
    }
    let scale = 10u128.checked_pow(18u32.checked_sub(frac.len() as u32)?)?;
    let frac = if frac.is_empty() { 0 } else { frac.parse::<u128>().ok()?.checked_mul(scale)? };
    whole.parse::<u128>().ok()?.checked_mul(1_000_000_000_000_000_000)?.checked_add(frac)
}
