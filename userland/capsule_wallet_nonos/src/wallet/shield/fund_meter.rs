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
 * A withdrawal to the meter (B7, the plan and the plan): the meter's ETH for
 * gas and its NOX for channels come out of the pool, so the meter is not
 * linked to this wallet's public address. Its payments still link to each
 * other: it is one pseudonym across payees. Shield-funded paths refuse
 * unless the pool the shield uses is the config's Shield pool on its chain,
 * and the meter must have its key open to be paid.
 */

use nonos_payment_proto::{Account, READY};

pub const NOT_THIS_POOL: &str = "The meter is funded only from the Shield pool this system's \
     config names on this chain, and the shield uses another.";
pub const METER_NOT_READY: &str = "The meter has no key open on this boot, so it cannot be \
     paid. A boot that keeps nothing holds no meter key.";
pub const NO_METER: &str = "The meter did not answer, so there is no address to fund.";

fn lower_hex(a: &[u8; 20]) -> [u8; 40] {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut out = [0u8; 40];
    for (i, b) in a.iter().enumerate() {
        out[2 * i] = HEX[(b >> 4) as usize];
        out[2 * i + 1] = HEX[(b & 15) as usize];
    }
    out
}

/// Whether `pool`, as the shield screens write it, is the config's Shield
/// pool on `chain_id`.
pub fn pool_is_configured(pool: &str, chain_id: u64) -> bool {
    let Ok(want) = nonos_nox_config::address(chain_id, nonos_nox_config::Contract::ShieldPool)
    else {
        return false;
    };
    let digits = pool.strip_prefix("0x").unwrap_or(pool).as_bytes();
    digits.len() == 40 && digits.iter().map(u8::to_ascii_lowercase).eq(lower_hex(&want))
}

/// The meter's address as the destination of a withdrawal, or why not.
pub fn meter_destination(
    pool: &str,
    chain_id: u64,
    account: Option<&Account>,
) -> Result<[u8; 20], &'static str> {
    if !pool_is_configured(pool, chain_id) {
        return Err(NOT_THIS_POOL);
    }
    let a = account.ok_or(NO_METER)?;
    if a.flags & READY == 0 || a.address == [0; 20] {
        return Err(METER_NOT_READY);
    }
    Ok(a.address)
}
