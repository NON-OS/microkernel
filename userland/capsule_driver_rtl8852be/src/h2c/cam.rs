// NONOS Operating System (AGPL-3.0-or-later)

//! The address and BSSID CAM entries of the one station interface,
//! `rtw89_fw_h2c_cam` with `rtw89_cam_fill_addr_cam_info` and
//! `rtw89_cam_fill_bssid_cam_info` (Linux v6.12 rtw89 `fw.c`, `cam.c`):
//! entry 0 of each, valid, no address mask, normal security mode with the
//! keys installed (`cam_keys`), the BSSID matched on all six bytes. Body
//! word 0 is unused, as rtw89's macros leave it.

use super::cam_keys::Keys;
use super::media::{Net, MAC_ID, PORT};
use super::packet::Cmd;
use super::send::{h2c, SendFail};
use super::Fw;
use crate::dma::DmaMem;
use crate::regs::{field, Mmio};
use crate::ring::tx::TxRing;

pub const CAM_UPD: Cmd = Cmd { cat: 1, class: 6, func: 0 };
pub const CAM_LEN: usize = 60;
const ADDR_CAM_ENT_SIZE: u32 = 0x40;
const BSSID_CAM_ENT_SIZE: u32 = 0x8;
const SEC_ENT_MODE_NORMAL: u32 = 2;
const BSSID_MATCH_ALL: u32 = 0x3f;

/// `rtw89_cam_addr_hash` from byte 0.
pub fn hash(a: &[u8; 6]) -> u32 {
    a.iter().fold(0u8, |h, b| h ^ b) as u32
}

fn bytes_le(a: &[u8]) -> u32 {
    a.iter().rev().fold(0, |w, &b| (w << 8) | b as u32)
}

/// The command body: `sma` is the address the station transmits from,
/// `bssid` its AP's (zero before one is chosen), `aid` its association ID,
/// `keys` the security entries.
pub fn cam_body(sma: &[u8; 6], bssid: &[u8; 6], net: Net, aid: u16, keys: Keys) -> [u8; CAM_LEN] {
    let mut w = [0u32; 15];
    w[1] = field(0xff_0000, ADDR_CAM_ENT_SIZE); // index 0, offset 0
    w[2] =
        1 | field(0x6, net as u32) | field(0xff_0000, hash(sma)) | field(0xff00_0000, hash(bssid));
    w[4] = bytes_le(&sma[..4]);
    w[5] = bytes_le(&sma[4..]) | (bytes_le(&bssid[..2]) << 16);
    w[6] = bytes_le(&bssid[2..]);
    w[8] = MAC_ID as u32 | field(0x700, PORT as u32) | field(0x3800, PORT as u32);
    if net == Net::Infra {
        w[9] = (aid & 0xfff) as u32;
    }
    let (k9, k10, k11) = keys.words();
    (w[9], w[10], w[11]) = (w[9] | field(0x3_0000, SEC_ENT_MODE_NORMAL) | k9, k10, k11);
    w[12] = field(0xff_0000, BSSID_CAM_ENT_SIZE); // BSSID entry 0, offset 0
    w[13] = 1 | field(0xfc, BSSID_MATCH_ALL) | (bytes_le(&bssid[..2]) << 16);
    w[14] = bytes_le(&bssid[2..]);
    let mut b = [0u8; CAM_LEN];
    for (i, v) in w.iter().enumerate() {
        b[4 * i..4 * i + 4].copy_from_slice(&v.to_le_bytes());
    }
    b
}

pub fn cam<M: DmaMem>(
    r: &impl Mmio,
    q: &mut TxRing<M>,
    fw: &mut Fw,
    body: &[u8; CAM_LEN],
) -> Result<(), SendFail> {
    h2c(r, q, fw, CAM_UPD, body, true)
}
