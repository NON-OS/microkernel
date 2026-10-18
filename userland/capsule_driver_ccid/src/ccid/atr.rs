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

//! An Answer-to-Reset as a power-on's DataBlock carries it: TS and T0 at
//! least, and at most 33 bytes, TS and 32 more (ISO/IEC 7816-3, as
//! recalled, unverified). A reader that hands up anything else has not
//! read a card's ATR, and the holder is not given it as one.

const ATR_MIN: usize = 2;
const ATR_MAX: usize = 33;

pub fn atr_fits(atr: &[u8]) -> bool {
    (ATR_MIN..=ATR_MAX).contains(&atr.len())
}
