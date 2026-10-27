// NONOS Operating System (AGPL-3.0-or-later)

//! The station's MAC ID tables and receive filter (Linux v6.12 rtw89):
//! `rtw89_mac_dmac_tbl_init` and `rtw89_mac_cmac_tbl_init` (`mac.c`,
//! written through the indirect access window), `rtw89_fw_h2c_macid_pause`
//! with the firmware's group format (this firmware lacks
//! MACID_PAUSE_SLEEP), and the receive filter `rtw89_ops_configure_filter`
//! writes (`mac80211.c`): rtw89's default, with the BSSID, broadcast and
//! address-1 checks dropped while scanning.

use crate::dma::DmaMem;
use crate::h2c::media::MAC_ID;
use crate::h2c::packet::Cmd;
use crate::h2c::send::{h2c_acks, SendFail};
use crate::h2c::Fw;
use crate::regs::{Mmio, Rw};
use crate::ring::tx::TxRing;

const R_AX_FILTER_MODEL_ADDR: u32 = 0xc04;
const R_AX_INDIR_ACCESS_ENTRY: u32 = 0x4_0000;
const DMAC_TBL_BASE: u32 = 0x1880_0000;
const CMAC_TBL_BASE: u32 = 0x1884_0000;
const CCTL_INFO_SIZE: u32 = 0x20;
/// What `rtw89_mac_cmac_tbl_init` writes, word by word.
pub const CMAC_INIT: [u32; 8] = [0x4, 0x400a_0004, 0, 0, 0, 0xe43_000b, 0, 0xb_8109];
pub const MACID_PAUSE: Cmd = Cmd { cat: 1, class: 9, func: 8 };
pub const R_AX_RX_FLTR_OPT: u32 = 0xce20;
/// `B_AX_RX_FLTR_CFG_MASK`: every bit but 21:16.
const FLTR_CFG_MASK: u32 = 0xffc0_ffff;
pub const DEFAULT_RX_FLTR: u32 = 0x0300_44be;
/// B_AX_A_BCN_CHK_EN, B_AX_A_BC, B_AX_A_A1_MATCH.
pub const SCAN_OPEN: u32 = 0x80 | 0x4 | 0x2;

pub fn tbl_init(r: &impl Mmio) {
    let id = MAC_ID as u32;
    for i in 0..4 {
        r.write32(R_AX_FILTER_MODEL_ADDR, DMAC_TBL_BASE + (id << 4) + (i << 2));
        r.write32(R_AX_INDIR_ACCESS_ENTRY, 0);
    }
    r.write32(R_AX_FILTER_MODEL_ADDR, CMAC_TBL_BASE + id * CCTL_INFO_SIZE);
    for (i, v) in CMAC_INIT.iter().enumerate() {
        r.write32(R_AX_INDIR_ACCESS_ENTRY + 4 * i as u32, *v);
    }
}

/// The body for MAC ID 0 unpaused: pause groups all clear, mask group 0
/// bit 0 set.
pub fn unpause_body() -> [u8; 32] {
    let mut b = [0u8; 32];
    b[16..20].copy_from_slice(&(1u32 << MAC_ID).to_le_bytes());
    b
}

pub fn unpause<M: DmaMem>(r: &impl Mmio, q: &mut TxRing<M>, fw: &mut Fw) -> Result<(), SendFail> {
    // rtw89 asks a receive acknowledgement of this one always.
    h2c_acks(r, q, fw, MACID_PAUSE, &unpause_body(), true, false)
}

pub fn rx_filter(r: &impl Mmio, scanning: bool) {
    let v = if scanning { DEFAULT_RX_FLTR & !SCAN_OPEN } else { DEFAULT_RX_FLTR };
    r.mask32(R_AX_RX_FLTR_OPT, FLTR_CFG_MASK, v);
}
