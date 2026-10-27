// NONOS Operating System (AGPL-3.0-or-later)

//! A frame's WD body and WD info, `rtw89_core_fill_txdesc` with its
//! `rtw89_build_txwd_*` helpers (Linux v6.12 rtw89 `core.c`). Every frame
//! this driver sends has the WD info (rtw89 sets `en_wd_info` for both
//! management and data), no hardware security (the station encrypts in
//! software), and no aggregation.

use crate::regs::field;

pub const WD_BODY: usize = 24;
pub const WD_INFO: usize = 24;
pub const WD_LEN: usize = WD_BODY + WD_INFO;
/// RTW89_TX_QSEL_B0_MGMT on DMA channel CH8, RTW89_TX_QSEL_BE_0 on ACH0.
pub const QSEL_MGMT: u32 = 0x12;
pub const CH_MGMT: u32 = 8;
pub const QSEL_BE: u32 = 0x0;
pub const CH_BE: u32 = 0;

/// What `rtw89_core_tx_update_desc_info` settles for one frame.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Desc {
    pub mgmt: bool,
    pub pkt_size: u16,
    /// The sequence number from the frame's header.
    pub seq: u16,
    pub is_bmc: bool,
    /// The 802.11 header length, data frames only (in bytes).
    pub hdr_len: u8,
    /// Management: the fixed rate. Data: the lowest retry rate.
    pub rate: u16,
    /// `bk` for EAPOL (`PACKET_EAPOL`).
    pub bk: bool,
}

pub fn build(d: &Desc) -> [u8; WD_LEN] {
    let (qsel, ch) = if d.mgmt { (QSEL_MGMT, CH_MGMT) } else { (QSEL_BE, CH_BE) };
    let ssn = if d.mgmt { 1 } else { 0 }; // RTW89_MGMT_HW_SSN_SEL / _SEQ_MODE
    let llc = if d.mgmt { 0 } else { d.hdr_len as u32 >> 1 };
    let body0 = field(0x40_0000, 1) // WD_INFO_EN
        | field(0xf_0000, ch)
        | field(0xf800, llc)
        | field(0x80, 1) // WD_PAGE
        | field(0xc, ssn)
        | field(0x3, ssn);
    let body2 = field(0x7e_0000, qsel) | field(0x3fff, d.pkt_size as u32); // TID 0, MAC ID 0
    let body3 = field(0xfff, d.seq as u32) | field(0x2000, d.bk as u32);
    let info0 = match d.mgmt {
        true => field(0x4000_0000, 1) | field(0x1ff_0000, d.rate as u32) | field(0x400, 1),
        false => 0, // the firmware's rate adaptation picks; port 0
    };
    let info1 = if d.mgmt { 0 } else { field(0x1ff_0000, d.rate as u32) };
    let info4 = field(0x800_0000, !d.is_bmc as u32) | field(0x8000_0000, 1); // RTS, HW_RTS
    let words = [body0, 0, body2, body3, 0, 0, info0, info1, 0, 0, info4, 0];
    let mut b = [0u8; WD_LEN];
    for (i, w) in words.iter().enumerate() {
        b[4 * i..4 * i + 4].copy_from_slice(&w.to_le_bytes());
    }
    b
}
