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
 * A top-up as typed: whole NOX for a channel of the meter's, into TOPUP's
 * body. Pure, so wallet_proofs reads it.
 */

use nonos_payment_proto::TopUp;

const WEI_PER_NOX: u128 = 1_000_000_000_000_000_000;
pub const NO_AMOUNT: &str = "Type the top-up in whole NOX, from 1 to 1,000,000,000,000.";

pub fn topup_request(
    chain_id: u64,
    channel_id: [u8; 32],
    text: &str,
) -> Result<TopUp, &'static str> {
    let digits = !text.is_empty() && text.len() <= 13 && text.bytes().all(|b| b.is_ascii_digit());
    let whole: u128 = if digits { text.parse().map_err(|_| NO_AMOUNT)? } else { 0 };
    if !(1..=1_000_000_000_000).contains(&whole) {
        return Err(NO_AMOUNT);
    }
    Ok(TopUp { chain_id, channel_id, amount: whole * WEI_PER_NOX })
}
