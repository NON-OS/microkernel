// NONOS Operating System (AGPL-3.0-or-later)

//! What the PHY code keeps between calls: the board's RFE type and cut,
//! the BB gain table (`rtwdev->bb_gain.ax`), and the efuse RX gain
//! corrections with the two RPL bases `__rtw8852bx_bb_sethw` reads after
//! the BB table (`struct rtw89_phy_efuse_gain`, Linux v6.12 rtw89
//! `core.h`).

use super::gain::BbGain;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct EfuseGain {
    /// `offset[path][rtw89_gain_offset]`, CCK and 2G OFDM to 5G high.
    pub offset: [[i8; 5]; 2],
    pub offset_valid: bool,
    /// `comp[path][rtw89_subband]`.
    pub comp: [[i8; 5]; 2],
    pub comp_valid: bool,
    /// S(8,4) values read back from R_P0_RPL1 and R_P1_RPL1.
    pub offset_base: i8,
    pub rssi_base: i8,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Phy {
    pub rfe: u8,
    pub cv: u8,
    pub gain: BbGain,
    pub efuse_gain: EfuseGain,
}
