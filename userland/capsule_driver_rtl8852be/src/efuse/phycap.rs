// NONOS Operating System (AGPL-3.0-or-later)

//! The PHY capability cells, 128 physical bytes at 0x580:
//! `__rtw8852bx_read_phycap` and its parsers (Linux v6.12 rtw89
//! `rtw8852b_common.c`): the power calibration mark, TSSI, thermal and PA
//! bias trims, and the gain compensation. A trim byte of 0xff is unset.

use super::map::nibble;

pub const PHYCAP_ADDR: u32 = 0x580;
pub const PHYCAP_SIZE: usize = 128;
const PWR_K_CHK: usize = 0x5e9;
const PWR_K_VALUE: u8 = 0xaa;
const TSSI_TRIM: [usize; 2] = [0x5d6, 0x5ab];
const THERMAL_TRIM: [usize; 2] = [0x5df, 0x5dc];
const PA_BIAS_TRIM: [usize; 2] = [0x5de, 0x5db];
/// Per path: 2G, 5G band 1, (band 2 unused), 5G band 3, band 4.
const GAIN_COMP: [[usize; 5]; 2] =
    [[0x5bb, 0x5ba, 0, 0x5b9, 0x5b8], [0x590, 0x58f, 0, 0x58e, 0x58d]];

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Phycap {
    pub power_k_valid: bool,
    pub tssi_trim: [[u8; 8]; 2],
    /// `None` when unprogrammed (`pg_thermal_trim` false).
    pub thermal_trim: Option<[u8; 2]>,
    pub pa_bias_trim: Option<[u8; 2]>,
    pub gain_comp: [[i8; 5]; 2],
    pub gain_comp_valid: bool,
}

pub fn parse(cap: &[u8; PHYCAP_SIZE]) -> Phycap {
    let at = |a: usize| cap[a - PHYCAP_ADDR as usize];
    let mut p = Phycap { power_k_valid: at(PWR_K_CHK) == PWR_K_VALUE, ..Phycap::default() };
    let mut pg = false;
    for (path, &base) in TSSI_TRIM.iter().enumerate() {
        for j in 0..8 {
            p.tssi_trim[path][j] = at(base - j);
            pg |= at(base - j) != 0xff;
        }
    }
    if !pg {
        p.tssi_trim = [[0; 8]; 2];
    }
    let pair = |a: [usize; 2]| {
        let v = [at(a[0]), at(a[1])];
        (v != [0xff, 0xff]).then_some(v)
    };
    p.thermal_trim = pair(THERMAL_TRIM);
    p.pa_bias_trim = pair(PA_BIAS_TRIM);
    for (path, addrs) in GAIN_COMP.iter().enumerate() {
        for (band, &a) in addrs.iter().enumerate().filter(|(_, &a)| a != 0) {
            p.gain_comp[path][band] = nibble(at(a) & 0xf);
            p.gain_comp_valid |= at(a) != 0xff;
        }
    }
    p
}
