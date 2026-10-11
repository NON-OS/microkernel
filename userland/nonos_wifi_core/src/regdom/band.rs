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

//! Channel numbers and the frequencies they name (IEEE Std 802.11-2020,
//! Annex E). A beacon names its channel by number only; on the 2.4 and 5 GHz
//! bands those numbers do not overlap, so the number alone picks the band.
//! 6 GHz reuses the small numbers and is only reached by a driver that says so.

/// A radio band.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Band {
    Ghz2,
    Ghz5,
    Ghz6,
}

impl Band {
    /// The band a legacy (2.4 or 5 GHz) channel number belongs to.
    pub const fn of_legacy(channel: u8) -> Self {
        if channel <= 14 {
            Band::Ghz2
        } else {
            Band::Ghz5
        }
    }
}

/// Every 2.4 GHz channel number, in sweep order.
pub const CHANNELS_2GHZ: [u8; 14] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14];
/// The 20 MHz 5 GHz channels any regulatory domain opens, in sweep order.
pub const CHANNELS_5GHZ: [u8; 28] = [
    36, 40, 44, 48, 52, 56, 60, 64, 100, 104, 108, 112, 116, 120, 124, 128, 132, 136, 140, 144,
    149, 153, 157, 161, 165, 169, 173, 177,
];

/// The centre frequency of `channel` on `band` in MHz, or `None` for a
/// number the band does not define.
pub const fn channel_mhz(band: Band, channel: u8) -> Option<u32> {
    let ch = channel as u32;
    match band {
        Band::Ghz2 => match channel {
            1..=13 => Some(2407 + 5 * ch),
            14 => Some(2484),
            _ => None,
        },
        Band::Ghz5 => match channel {
            1..=196 => Some(5000 + 5 * ch),
            _ => None,
        },
        Band::Ghz6 => match channel {
            2 => Some(5935),
            1..=233 if channel % 4 == 1 => Some(5950 + 5 * ch),
            _ => None,
        },
    }
}
