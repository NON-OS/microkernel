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

//! The Request Data Object (USB Power Delivery 3, as recalled,
//! unverified): which of the charger's offers this machine took, by its
//! position from 1 (31:28), and what it asked of it: for a fixed or
//! variable supply the operating current (19:10, 10 mA), for a battery
//! supply the operating power (19:10, 250 mW), for a programmable one the
//! output voltage (20:9, 20 mV) and current (6:0, 50 mA).

use super::pdo::{bits, pdo, Pdo};

/// What was taken: each figure the request states, 0 where it does not.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Contract {
    pub position: u8,
    pub mv: u32,
    pub ma: u32,
    pub mw: u32,
}

/// The request read against the offers it names by position (from 1);
/// None for a position no offer holds.
pub fn contract(rdo: u32, offered: &[u32]) -> Option<Contract> {
    let position = bits(rdo, 28, 4) as u8;
    let p = pdo(*offered.get(usize::from(position).checked_sub(1)?)?);
    let (mv, ma, mw) = match p {
        Pdo::Fixed { mv, .. } => (mv, bits(rdo, 10, 10) * 10, mv * bits(rdo, 10, 10) * 10 / 1000),
        Pdo::Variable { .. } => (0, bits(rdo, 10, 10) * 10, 0),
        Pdo::Battery { .. } => (0, 0, bits(rdo, 10, 10) * 250),
        Pdo::Pps { .. } => {
            let (mv, ma) = (bits(rdo, 9, 12) * 20, bits(rdo, 0, 7) * 50);
            (mv, ma, mv * ma / 1000)
        }
        Pdo::Other => (0, 0, 0),
    };
    Some(Contract { position, mv, ma, mw })
}
