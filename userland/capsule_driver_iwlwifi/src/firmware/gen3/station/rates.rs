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

//! The legacy rates of one BSS, from the rate octets its beacon carries
//! (Supported Rates then Extended Supported Rates, in 500 kb/s units, the
//! top bit marking a basic rate).
//!
//! The ACK rate bitmaps the link context takes are Linux v6.12
//! mvm/mac-ctxt.c `iwl_mvm_ack_rates`: the basic rates as bits of the CCK set
//! (1, 2, 5.5, 11) and the OFDM set (6 to 54), then every mandatory rate below
//! the lowest basic one of its class (24, 12 and always 6 Mb/s; 11, 5.5, 2
//! and always 1 Mb/s). A 5 GHz band has no CCK rates, so a CCK octet there is
//! ignored as mac80211's 5 GHz rate table would.
//!
//! Frames this driver hands the firmware carry their rate
//! (`IWL_TX_FLAGS_CMD_RATE`): no rate scaling table is configured. Management
//! and EAPOL frames go at the lowest basic rate, preferring CCK on 2.4 GHz
//! (`iwl_mvm_mac_ctxt_get_lowest_rate`); data frames at the highest basic
//! rate, the fastest one every station of the BSS must decode. The rate word
//! is the version 2 format TX_CMD versions above 8 take (fw/api/rs.h): the
//! legacy rate's index within its class in bits 0-3
//! (`iwl_mvm_mac80211_idx_to_hwrate`), the modulation in bits 8-10
//! (`RATE_MCS_CCK_MSK` 0, `RATE_MCS_LEGACY_OFDM_MSK` 1 << 8), and the antenna
//! in bits 14-15 (`RATE_MCS_ANT_POS`).

/// The legacy rates in Linux's index order (`IWL_RATE_1M_INDEX` 0 to
/// `IWL_RATE_54M_INDEX` 11), in 500 kb/s units. Indices below
/// [`FIRST_OFDM`] are CCK.
pub const RATES: [u8; 12] = [2, 4, 11, 22, 12, 18, 24, 36, 48, 72, 96, 108];
/// `IWL_FIRST_OFDM_RATE`.
pub const FIRST_OFDM: usize = 4;
const BASIC: u8 = 0x80;
const RATE_MCS_LEGACY_OFDM: u32 = 1 << 8;
const RATE_MCS_ANT_POS: u32 = 14;

/// What the link and the transmit path need from one BSS's rates.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct BssRates {
    /// `cck_rates` and `ofdm_rates` of the link context.
    pub cck_ack: u8,
    pub ofdm_ack: u8,
    /// The rate index management and EAPOL frames go at.
    pub mgmt: usize,
    /// The rate index data frames go at.
    pub data: usize,
}

// The index of a rate octet in `RATES`, if it is a legacy rate this band has.
fn index(octet: u8, band_24: bool) -> Option<usize> {
    let i = RATES.iter().position(|&r| r == octet & !BASIC)?;
    (band_24 || i >= FIRST_OFDM).then_some(i)
}

/// The rates of a BSS on `channel` whose beacon offers `octets`.
pub fn bss_rates(octets: &[u8], channel: u8) -> BssRates {
    let band_24 = channel <= 14;
    let mut cck = 0u8;
    let mut ofdm = 0u8;
    let mut lowest_cck = usize::MAX;
    let mut lowest_ofdm = usize::MAX;
    let mut highest: Option<usize> = None;
    for &o in octets.iter().filter(|&&o| o & BASIC != 0) {
        let Some(i) = index(o, band_24) else { continue };
        if i >= FIRST_OFDM {
            ofdm |= 1 << (i - FIRST_OFDM);
            lowest_ofdm = lowest_ofdm.min(i);
        } else {
            cck |= 1 << i;
            lowest_cck = lowest_cck.min(i);
        }
        highest = Some(highest.map_or(i, |h| h.max(i)));
    }
    // Mandatory rates below the lowest basic one of each class.
    for (rate, bit) in [(8usize, 4u8), (6, 2)] {
        if rate < lowest_ofdm {
            ofdm |= 1 << bit;
        }
    }
    ofdm |= 1;
    for rate in [3usize, 2, 1] {
        if rate < lowest_cck {
            cck |= 1 << rate;
        }
    }
    cck |= 1;
    let mgmt = if band_24 && lowest_cck != usize::MAX {
        lowest_cck
    } else if lowest_ofdm != usize::MAX {
        lowest_ofdm
    } else if band_24 {
        0
    } else {
        FIRST_OFDM
    };
    BssRates { cck_ack: cck, ofdm_ack: ofdm, mgmt, data: highest.unwrap_or(mgmt) }
}

/// The version 2 rate word for legacy rate `index` on the lowest antenna
/// set in `tx_ant`.
pub fn rate_n_flags(index: usize, tx_ant: u8) -> u32 {
    let index = index.min(RATES.len() - 1);
    let (code, modulation) =
        if index >= FIRST_OFDM { (index - FIRST_OFDM, RATE_MCS_LEGACY_OFDM) } else { (index, 0) };
    let ant = if tx_ant & 0x3 == 0 { 1 } else { tx_ant & tx_ant.wrapping_neg() & 0x3 };
    code as u32 | modulation | (u32::from(ant) << RATE_MCS_ANT_POS)
}
