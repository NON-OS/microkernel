// NONOS Operating System (AGPL-3.0-or-later)

//! The BB gain table's values: `rtw89_phy_config_bb_gain_ax` with
//! `rtw89_phy_cfg_bb_gain_error` and `rtw89_phy_cfg_bb_rpl_ofst` (Linux
//! v6.12 rtw89 `phy.c`). An entry's address packs the config type, the gain
//! band, the path and a type byte (for RPL offsets: bandwidth in the high
//! nibble, first subchannel in the low). The 8852B's table holds gain
//! errors and RPL offsets for the 2 GHz and three 5 GHz gain bands; the
//! bypass and OP1dB kinds rtw89 also parses are kept by nothing this
//! driver writes, and are not stored.

pub const BANDS: usize = 4;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct BbGain {
    pub lna: [[[i8; 7]; 2]; BANDS],
    pub tia: [[[i8; 2]; 2]; BANDS],
    pub rpl_20: [[i8; 2]; BANDS],
    pub rpl_40: [[[i8; 9]; 2]; BANDS],
    pub rpl_80: [[[i8; 13]; 2]; BANDS],
}

impl BbGain {
    /// Take one table entry.
    pub fn take(&mut self, addr: u32, data: u32) {
        let (ty, path, band, cfg) =
            (addr as u8, (addr >> 8) as u8 as usize, (addr >> 16) as u8 as usize, addr >> 24);
        if band >= BANDS || path >= 2 || (0xf9..=0xfe).contains(&addr) {
            return;
        }
        let bytes = data.to_le_bytes().map(|b| b as i8);
        match (cfg, ty) {
            (0, 0) => self.lna[band][path][..4].copy_from_slice(&bytes),
            (0, 1) => self.lna[band][path][4..].copy_from_slice(&bytes[..3]),
            (0, 2) => self.tia[band][path].copy_from_slice(&bytes[..2]),
            (1, _) => self.rpl(band, path, ty >> 4, ty & 0xf, data),
            _ => {}
        }
    }

    fn rpl(&mut self, band: usize, path: usize, bw: u8, start: u8, data: u32) {
        if bw == 0 {
            self.rpl_20[band][path] = data as i8;
            return;
        }
        let b = data.to_le_bytes().map(|b| b as i8);
        let (s, n) = (
            start as usize,
            match (bw, start) {
                (_, 0) => 1,
                (1, 1) => 2,
                (2, 1) => 4,
                (2, 9) => 2,
                _ => return,
            },
        );
        match bw {
            1 => self.rpl_40[band][path][s..s + n].copy_from_slice(&b[..n]),
            2 => self.rpl_80[band][path][s..s + n].copy_from_slice(&b[..n]),
            _ => {}
        }
    }
}
