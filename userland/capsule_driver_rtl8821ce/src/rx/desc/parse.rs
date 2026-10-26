// NONOS Operating System (AGPL-3.0-or-later)

//! Parse the RX descriptor the card prepends to each received frame, and build
//! the descriptor that re-arms a consumed RX slot. The RX descriptor's first
//! word carries the frame length, the driver-info size (in 8-byte units) and a
//! shift, which together give the offset from the descriptor to the frame, plus
//! the CRC and ICV error flags and whether the frame is a firmware command
//! (C2H) rather than an 802.11 frame, and the security type with the
//! software-decrypt flag, which together say whether the chip decrypted a
//! protected frame (rtw88 `rtw_rx_query_rx_desc`: decrypted = !SWDEC and
//! ENC_TYPE != none). Bit positions follow the rtw88 `rx.h` `RTW_RX_DESC_W*`
//! fields; checked against known-answer descriptors in `rtl8821ce_proofs`.

use super::super::regs::RX_PKT_DESC_SIZE;
use super::fields::*;
use super::info::RxInfo;

/// Parse the descriptor at the start of `buf`, or `None` if `buf` is shorter
/// than one descriptor.
pub fn parse(buf: &[u8]) -> Option<RxInfo> {
    if buf.len() < RX_PKT_DESC_SIZE {
        return None;
    }
    let w0 = u32::from_le_bytes([buf[0], buf[1], buf[2], buf[3]]);
    let w2 = u32::from_le_bytes([buf[8], buf[9], buf[10], buf[11]]);
    Some(RxInfo {
        pkt_len: (w0 & W0_PKT_LEN) as usize,
        drv_info_size: ((w0 >> W0_DRV_INFO_SHIFT) & W0_DRV_INFO_MASK) as usize * 8,
        shift: ((w0 >> W0_SHIFT_SHIFT) & W0_SHIFT_MASK) as usize,
        phy_status: w0 & W0_PHYST != 0,
        crc_err: w0 & W0_CRC32 != 0,
        icv_err: w0 & W0_ICV_ERR != 0,
        is_c2h: w2 & W2_C2H != 0,
        enc_type: ((w0 >> W0_ENC_TYPE_SHIFT) & W0_ENC_TYPE_MASK) as u8,
        swdec: w0 & W0_SWDEC != 0,
    })
}
