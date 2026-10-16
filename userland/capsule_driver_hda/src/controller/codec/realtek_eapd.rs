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

//! `alc_fill_eapd_coef`'s table: the coefficient bit each ALC part needs
//! changed before its EAPD pin drives the external amplifier.

use super::realtek_coef::{upd, CoefUpdate};

/// `alc_fill_eapd_coef` for codec `vendor_id`, whose coefficient 0 reads
/// `coef0`. At most three updates apply to one codec.
pub fn eapd_coef(vendor_id: u32, coef0: u16) -> [Option<CoefUpdate>; 3] {
    let variant = coef0 & 0x00f0;
    match vendor_id {
        0x10ec_0262 => [Some(upd(0x7, 0, 1 << 5)), None, None],
        0x10ec_0267 | 0x10ec_0268 => [Some(upd(0x7, 0, 1 << 13)), None, None],
        0x10ec_0269 => match variant {
            0x10 => [Some(upd(0xd, 0, 1 << 14)), None, None],
            0x20 => [Some(upd(0x4, 1 << 15, 0)), None, None],
            0x30 => [Some(upd(0x10, 1 << 9, 0)), None, None],
            _ => [None, None, None],
        },
        0x10ec_0280 | 0x10ec_0284 | 0x10ec_0290 | 0x10ec_0292 => {
            [Some(upd(0x4, 1 << 15, 0)), None, None]
        }
        0x10ec_0225 | 0x10ec_0295 | 0x10ec_0299 => [
            Some(upd(0x67, 0xf000, 0x3000)),
            Some(upd(0x36, 1 << 13, 0)),
            Some(upd(0x10, 1 << 9, 0)),
        ],
        0x10ec_0215 | 0x10ec_0285 | 0x10ec_0289 => {
            [Some(upd(0x36, 1 << 13, 0)), Some(upd(0x10, 1 << 9, 0)), None]
        }
        0x10ec_0230 | 0x10ec_0233 | 0x10ec_0235 | 0x10ec_0236 | 0x10ec_0245 | 0x10ec_0255
        | 0x10ec_0256 | 0x19e5_8326 | 0x10ec_0257 | 0x10ec_0282 | 0x10ec_0283 | 0x10ec_0286
        | 0x10ec_0288 | 0x10ec_0298 | 0x10ec_0300 => [Some(upd(0x10, 1 << 9, 0)), None, None],
        0x10ec_0275 => [Some(upd(0xe, 0, 1)), None, None],
        0x10ec_0287 => [Some(upd(0x10, 1 << 9, 0)), Some(upd(0x8, 0xffff, 0x4ab7)), None],
        0x10ec_0293 => [Some(upd(0xa, 1 << 13, 0)), None, None],
        0x10ec_0234 | 0x10ec_0274 | 0x10ec_0294 | 0x10ec_0700 | 0x10ec_0701 | 0x10ec_0703
        | 0x10ec_0711 => [Some(upd(0x10, 1 << 15, 0)), None, None],
        0x10ec_0662 if variant == 0x30 => [Some(upd(0x4, 1 << 10, 0)), None, None],
        0x10ec_0272 | 0x10ec_0273 | 0x10ec_0663 | 0x10ec_0665 | 0x10ec_0670 | 0x10ec_0671
        | 0x10ec_0672 => [Some(upd(0xd, 0, 1 << 14)), None, None],
        0x10ec_0222 | 0x10ec_0623 => [Some(upd(0x19, 1 << 13, 0)), None, None],
        0x10ec_0668 => [Some(upd(0x7, 3 << 13, 0)), None, None],
        0x10ec_0867 => [Some(upd(0x4, 1 << 10, 0)), None, None],
        0x10ec_0888 if variant == 0x20 || variant == 0x30 => {
            [Some(upd(0x7, 1 << 5, 0)), None, None]
        }
        0x10ec_0892 | 0x10ec_0897 => [Some(upd(0x7, 1 << 5, 0)), None, None],
        0x10ec_0899 | 0x10ec_0900 | 0x10ec_0b00 | 0x10ec_1168 | 0x10ec_1220 => {
            [Some(upd(0x7, 1 << 1, 0)), None, None]
        }
        _ => [None, None, None],
    }
}
