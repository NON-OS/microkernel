// NONOS Operating System (AGPL-3.0-or-later)

//! The firmware's rate adaptation for the AP, `rtw89_phy_ra_assoc` with
//! `rtw89_phy_ra_sta_update` and `rtw89_fw_h2c_ra` (Linux v6.12 rtw89
//! `phy.c`, `fw.c`), for what this station joins as: legacy rates only,
//! 20 MHz, one stream. The rate mask comes from the AP's supported rates
//! as mac80211 indexes them per band. The station does not track the AP's
//! RSSI, so it is 0, as rtw89's average is before any frame: every rate
//! stays allowed and the initial rate level is 0.

use super::media::MAC_ID;
use super::packet::Cmd;
use super::send::{h2c, SendFail};
use super::Fw;
use crate::dma::DmaMem;
use crate::regs::{field, Mmio};
use crate::ring::tx::TxRing;
use crate::tx::rate::bitmaps;

pub const RA_MACIDCFG: Cmd = Cmd { cat: 2, class: 1, func: 0 };
const MODE_CCK: u32 = 0x1;
const MODE_OFDM: u32 = 0x2;
const MASK_OFDM: u64 = 0xff0;
const MASK_SUBCCK: u64 = 0x5;
const MASK_LEGACY: u64 = 0xfff;

/// `supp_rates` for `rates` (bit 7, the basic flag, ignored), then the
/// rate mask and RA mode `rtw89_phy_ra_sta_update` derives.
pub fn mask_and_mode(rates: &[u8], five: bool) -> (u64, u32) {
    let supp = bitmaps(rates, five).0 as u64;
    let (mut mask, mode) = match five {
        true => (supp << 4, MODE_OFDM),
        false => {
            let cck = if supp & 0xf != 0 { MODE_CCK } else { 0 };
            (supp, cck | if supp & 0xff0 != 0 { MODE_OFDM } else { 0 })
        }
    };
    let bak = mask;
    if mode & MODE_OFDM != 0 {
        mask &= MASK_OFDM | MASK_SUBCCK;
    }
    // rtw89_phy_ra_mask_rssi at RSSI 0 keeps every rate.
    if mask & !MASK_LEGACY == 0 {
        mask |= bak & !MASK_LEGACY;
    }
    if mask == 0 {
        mask |= bak & MASK_LEGACY;
    }
    (mask, mode)
}

pub fn ra_body(mask: u64, mode: u32) -> [u8; 16] {
    let w0 = field(0x3e, mode) | field(0xff00, MAC_ID as u32) | (1 << 20); // UPD_ALL
    let w = [w0, mask as u32, field(0x7fff_ffff, (mask >> 32) as u32), 0];
    let mut b = [0u8; 16];
    for (i, v) in w.iter().enumerate() {
        b[4 * i..4 * i + 4].copy_from_slice(&v.to_le_bytes());
    }
    b
}

pub fn ra_assoc<M: DmaMem>(
    r: &impl Mmio,
    q: &mut TxRing<M>,
    fw: &mut Fw,
    rates: &[u8],
    five: bool,
) -> Result<(), SendFail> {
    let (mask, mode) = mask_and_mode(rates, five);
    h2c(r, q, fw, RA_MACIDCFG, &ra_body(mask, mode), false)
}
