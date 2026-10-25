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
 * The Shield's answer to OP_FRESH_ADDRESS read as an address (plan/27,
 * v1.4.52): forty hex digits, 0x or not, from a reply that succeeded.
 * No address, a short or garbled one, or the zero address is refused.
 */

use shield_wire::{field, STATUS_OK};

use crate::wallet::event::hex_digit::hex_digit;

pub const NO_FRESH: &str = "The shield gave no fresh address for a refund, so nothing was ordered.";

pub fn fresh_from(status: i32, body: &str) -> Result<[u8; 20], &'static str> {
    let text = field(body, "address").filter(|_| status == STATUS_OK).ok_or(NO_FRESH)?;
    let hex = text.strip_prefix("0x").unwrap_or(text).as_bytes();
    if hex.len() != 40 {
        return Err(NO_FRESH);
    }
    let mut out = [0u8; 20];
    for (i, b) in out.iter_mut().enumerate() {
        let hi = hex_digit(hex[2 * i] as u32).ok_or(NO_FRESH)?;
        let lo = hex_digit(hex[2 * i + 1] as u32).ok_or(NO_FRESH)?;
        *b = (hi << 4) | lo;
    }
    (out != [0; 20]).then_some(out).ok_or(NO_FRESH)
}
