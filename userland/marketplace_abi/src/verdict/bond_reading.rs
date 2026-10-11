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

//! A bond as settle's READ returns it (plan/24): bondOf's five words (payee,
//! publisherKey, amount net of slashes, unlockAt, slashed) and isActive's
//! one, read strictly (an address or a uint64 with high bytes, a bool other
//! than 0 or 1, is no reading), with the time it was read.

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct BondReading {
    pub payee: [u8; 20],
    pub publisher_key: [u8; 32],
    pub amount: u128,
    pub unlock_at: u64,
    pub slashed: u128,
    pub active: bool,
    pub read_at: u64,
}

fn num(w: &[u8; 32], bytes: usize) -> Option<u128> {
    let high = 32 - bytes;
    w[..high]
        .iter()
        .all(|b| *b == 0)
        .then(|| w[high..].iter().fold(0u128, |v, b| v << 8 | *b as u128))
}

impl BondReading {
    pub fn from_words(bond_of: &[[u8; 32]], is_active: &[[u8; 32]], read_at: u64) -> Option<Self> {
        let [payee, key, amount, unlock, slashed] = bond_of else { return None };
        let [active] = is_active else { return None };
        let active = match num(active, 1)? {
            0 => false,
            1 => true,
            _ => return None,
        };
        num(payee, 20)?;
        Some(BondReading {
            payee: payee[12..].try_into().ok()?,
            publisher_key: *key,
            amount: num(amount, 16)?,
            unlock_at: num(unlock, 8)? as u64,
            slashed: num(slashed, 16)?,
            active,
            read_at,
        })
    }
}
