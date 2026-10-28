// NONOS Operating System (AGPL-3.0-or-later)

//! The 8852B's logical efuse fields this driver uses, at their offsets in
//! `struct rtw8852bx_efuse` (Linux v6.12 rtw89 `rtw8852b_common.h`), read
//! as `__rtw8852bx_read_efuse` reads them. The station address the efuse
//! carries is never read: the station's address is drawn at random.

pub const LOGICAL_SIZE: usize = 2048;
pub const DAV_LOGICAL_SIZE: usize = 16;
pub const PHYSICAL_SIZE: usize = 1216;
pub const DAV_PHYSICAL_SIZE: usize = 96;
pub const SEC_CTRL_SIZE: usize = 4;
pub const PATH_A_TSSI: usize = 0x210;
pub const PATH_B_TSSI: usize = 0x23a;
pub const XTAL_K: usize = 0x2b9;
pub const RFE_TYPE: usize = 0x2ca;
pub const PATH_A_THERM: usize = 0x2d0;
pub const PATH_B_THERM: usize = 0x2d1;
/// rx_gain_2g_cck, _2g_ofdm, _5g_low, _5g_mid, _5g_high: one byte each,
/// path A in the high nibble, path B in the low (`_decode_efuse_gain`).
pub const RX_GAIN: [usize; 5] = [0x2d6, 0x2d4, 0x2d8, 0x2da, 0x2dc];

/// What the efuse says about this board.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Efuse {
    pub rfe_type: u8,
    pub xtal_k: u8,
    pub therm: [u8; 2],
    /// TSSI offsets per path: 6 CCK groups, 5 + 14 OFDM groups.
    pub tssi_cck: [[u8; 6]; 2],
    pub tssi_mcs: [[u8; 19]; 2],
    /// The RX gain offsets per path and gain band (CCK, 2G OFDM, 5G low,
    /// mid, high), signed, and whether any byte was programmed.
    pub gain: [[i8; 5]; 2],
    pub gain_valid: bool,
}

/// `sign_extend32(x, 3)` of a nibble.
pub const fn nibble(x: u8) -> i8 {
    ((x << 4) as i8) >> 4
}

pub fn parse(log: &[u8]) -> Option<Efuse> {
    let at = |i: usize| log.get(i).copied();
    let tssi = |base: usize| -> Option<([u8; 6], [u8; 19])> {
        let t = log.get(base..base + 32)?;
        let mut mcs = [0u8; 19];
        mcs[..5].copy_from_slice(&t[6..11]);
        mcs[5..].copy_from_slice(&t[18..32]);
        Some((t[..6].try_into().ok()?, mcs))
    };
    let (a, b) = (tssi(PATH_A_TSSI)?, tssi(PATH_B_TSSI)?);
    let mut gain = [[0i8; 5]; 2];
    let mut gain_valid = false;
    for (band, &off) in RX_GAIN.iter().enumerate() {
        let v = at(off)?;
        gain[0][band] = nibble(v >> 4);
        gain[1][band] = nibble(v & 0xf);
        gain_valid |= v != 0xff;
    }
    Some(Efuse {
        rfe_type: at(RFE_TYPE)?,
        xtal_k: at(XTAL_K)?,
        therm: [at(PATH_A_THERM)?, at(PATH_B_THERM)?],
        tssi_cck: [a.0, b.0],
        tssi_mcs: [a.1, b.1],
        gain,
        gain_valid,
    })
}
