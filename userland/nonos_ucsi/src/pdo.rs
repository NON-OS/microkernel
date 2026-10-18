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

//! Power Data Objects (USB Power Delivery 3, as recalled, unverified):
//! what a charger offers. Pure decoding; nothing here asks for power.

/// One offer: voltages in mV, current in mA, power in mW.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pdo {
    Fixed { mv: u32, ma: u32 },
    Battery { min_mv: u32, max_mv: u32, mw: u32 },
    Variable { min_mv: u32, max_mv: u32, ma: u32 },
    /// A programmable supply, the augmented kind with subtype 0.
    Pps { min_mv: u32, max_mv: u32, ma: u32 },
    /// An augmented kind this decoder does not know.
    Other,
}

pub(crate) fn bits(w: u32, at: u32, n: u32) -> u32 {
    (w >> at) & ((1 << n) - 1)
}

pub fn pdo(w: u32) -> Pdo {
    let (hi, lo) = (bits(w, 20, 10) * 50, bits(w, 10, 10) * 50);
    match w >> 30 {
        0 => Pdo::Fixed { mv: lo, ma: bits(w, 0, 10) * 10 },
        1 => Pdo::Battery { min_mv: lo, max_mv: hi, mw: bits(w, 0, 10) * 250 },
        2 => Pdo::Variable { min_mv: lo, max_mv: hi, ma: bits(w, 0, 10) * 10 },
        _ if bits(w, 28, 2) == 0 => Pdo::Pps {
            min_mv: bits(w, 8, 8) * 100,
            max_mv: bits(w, 17, 8) * 100,
            ma: bits(w, 0, 7) * 50,
        },
        _ => Pdo::Other,
    }
}

/// The most power the offer gives, in mW.
pub fn max_mw(p: Pdo) -> u32 {
    match p {
        Pdo::Fixed { mv, ma } => mv * ma / 1000,
        Pdo::Battery { mw, .. } => mw,
        Pdo::Variable { max_mv, ma, .. } | Pdo::Pps { max_mv, ma, .. } => max_mv * ma / 1000,
        Pdo::Other => 0,
    }
}
