// NONOS Operating System (AGPL-3.0-or-later)

//! A 20 MHz channel as rtw89 describes it (`rtw89_chan_create`, Linux v6.12
//! rtw89 `chan.c`): band, subband (`rtw89_get_subband_type`), the gain
//! bands derived from it (`phy.h`), and the BB's channel index
//! (`rtw89_encode_chan_idx`, `phy.c`). The station only ever tunes 20 MHz,
//! so primary and centre channel are the same.

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Chan {
    pub five: bool,
    pub ch: u8,
}

/// rtw89's 5 GHz channel base table, `rtw89_ch_base_table[2..=5]`.
const BASE_5G: [u8; 4] = [36, 100, 132, 149];

impl Chan {
    /// A channel this driver tunes, or `None`.
    pub const fn new(five: bool, ch: u8) -> Option<Chan> {
        let ok = match five {
            false => ch >= 1 && ch <= 14,
            true => ch >= 36 && ch <= 177,
        };
        if ok {
            Some(Chan { five, ch })
        } else {
            None
        }
    }

    /// `enum rtw89_subband`: 0 for 2 GHz, 1, 3 or 4 for the 5 GHz bands.
    pub const fn subband(self) -> u8 {
        match (self.five, self.ch) {
            (false, _) => 0,
            (true, 100..=144) => 3,
            (true, 149..=177) => 4,
            (true, _) => 1,
        }
    }

    /// `rtw89_subband_to_bb_gain_band`: 2G, 5G low, mid, high.
    pub const fn gain_band(self) -> usize {
        match self.subband() {
            0 => 0,
            1 => 1,
            3 => 2,
            _ => 3,
        }
    }

    /// `rtw89_subband_to_gain_offset_band_of_ofdm`: index into the efuse's
    /// RX gain offsets (0 is CCK, 1 2G OFDM, then 5G low, mid, high).
    pub const fn ofdm_offset_band(self) -> usize {
        self.gain_band() + 1
    }

    /// `rtw89_encode_chan_idx`.
    pub fn chan_idx(self) -> u8 {
        if !self.five {
            return self.ch & 0xf;
        }
        let i = BASE_5G.iter().rposition(|&b| self.ch >= b).unwrap_or(0);
        (((i + 2) as u8) << 4) | (((self.ch - BASE_5G[i]) >> 1) & 0xf)
    }
}
