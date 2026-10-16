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

//! Set the transmit power. Until this ran, frames left the antenna at the
//! power-on default of essentially zero, so no access point could hear them.
//! rtw8821c programs a per-rate power index, in 0.5 dB steps, into the TXAGC
//! registers: path A starts at 0x1D00, and each 32-bit register packs four
//! consecutive rates, one index per byte
//! (`rtw8821c_set_tx_power_index_by_rate`). The full driver derives each index
//! from the board's efuse power-by-rate tables and caps it at the regulatory
//! limit in the same units; until that calibration is ported, one index is
//! written for every rate the association and data path use: the lowest
//! world-domain EIRP limit (`nonos_wifi_core::regdom`) of any 2.4 GHz channel
//! the domain allows, less an assumed antenna gain. It is written once, at
//! bring-up, and holds across every retune, so it fits each channel. The
//! register program is checked against a modeled device in
//! `rtl8821ce_proofs`.

use nonos_wifi_core::regdom::max_eirp_dbm;

use crate::regs::Mmio;

/// TXAGC path-A base. Each register packs four rates, one power index per byte.
const REG_TXAGC_A: usize = 0x1D00;
/// The antenna gain assumed between the conducted power and the EIRP the
/// domain limits: 2 dBi, typical of a laptop's internal antennas. A board with
/// more gain needs its own figure.
const ANTENNA_GAIN_DBI: u8 = 2;
/// Rate groups to program: CCK (1 to 11M), the two OFDM groups (6 to 54M), and
/// the two single-stream MCS groups (MCS0 to MCS7). These are the rates the
/// authentication, handshake and data frames are sent at.
const RATE_GROUPS: u32 = 5;

/// The power index for `channel`: the domain's EIRP limit less the antenna
/// gain, in 0.5 dB steps; `None` where the domain does not allow the channel.
pub fn power_index(channel: u8) -> Option<u8> {
    max_eirp_dbm(channel)?.checked_sub(ANTENNA_GAIN_DBI)?.checked_mul(2)
}

/// The index written: the lowest of any 2.4 GHz channel's, so it fits each.
pub fn band_index() -> u8 {
    (1..=14).filter_map(power_index).min().unwrap_or(0)
}

/// Write the band's power index across the CCK, OFDM and single-stream MCS
/// rate registers for path A, so transmitted frames carry real power.
pub fn set_tx_power<M: Mmio>(mmio: &M) {
    let word = u32::from_le_bytes([band_index(); 4]);
    for group in 0..RATE_GROUPS {
        mmio.write32(REG_TXAGC_A + (group * 4) as usize, word);
    }
}
