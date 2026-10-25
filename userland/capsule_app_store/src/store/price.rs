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

//! The price line on a listing's card (B9, plan/23): what one unit costs in
//! NOX and how it settles, shown only when the market says the publisher and
//! the payee both stand behind it. The amount is exact, never rounded.

use alloc::vec::Vec;

pub const FREE: u8 = 0;
pub const VOUCHER: u8 = 1;
pub const ESCROW: u8 = 2;
const WEI: u128 = 1_000_000_000_000_000_000;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Price {
    pub mode: u8,
    /// The unit's label, as nonos_nox_config names it, or b"unit".
    pub unit: &'static [u8],
    pub unit_price: u128,
    /// The commerce block's payee: what the wallet's consent pays to.
    pub payee: [u8; 20],
    /// Commerce matches and the payee's binding is valid.
    pub trusted: bool,
}

pub fn price_line(p: &Price) -> Vec<u8> {
    if p.mode == FREE {
        return b"Free".to_vec();
    }
    if !p.trusted {
        return b"Price not confirmed by its payee".to_vec();
    }
    let how: &[u8] = match p.mode {
        VOUCHER => b", paid by voucher",
        ESCROW => b", bought through escrow",
        _ => b"",
    };
    [&nox_amount(p.unit_price)[..], b" NOX per ", p.unit, how].concat()
}

/// `wei` in NOX: the whole part, then every significant decimal.
/// v1.4.7: a voucher install drains to the chain first, at the meter's gas.
pub fn gas_line(p: &Price) -> Option<&'static [u8]> {
    (p.mode == VOUCHER && p.trusted)
        .then_some(b"Each install also spends one settle's gas from the meter's ETH")
}

pub fn nox_amount(wei: u128) -> Vec<u8> {
    let mut out = alloc::format!("{}", wei / WEI).into_bytes();
    let frac = alloc::format!("{:018}", wei % WEI);
    let frac = frac.trim_end_matches('0');
    if !frac.is_empty() {
        out.push(b'.');
        out.extend_from_slice(frac.as_bytes());
    }
    out
}
