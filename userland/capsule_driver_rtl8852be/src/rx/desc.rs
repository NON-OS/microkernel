// NONOS Operating System (AGPL-3.0-or-later)

//! The receive descriptor after a buffer's info word,
//! `rtw89_core_query_rxdesc` (Linux v6.12 rtw89 `core.c`): the packet's
//! length and type, the driver-info length that sits between descriptor
//! and packet, and the error and decryption flags. rtw89 never sets the
//! descriptor's shift, so it is 0 here too.

use crate::ring::bd::RXBD_INFO_SIZE;

pub const SHORT: usize = 16;
pub const LONG: usize = 32;
/// `enum rtw89_core_rx_type`'s 802.11 frame; the others (PPDU status 1,
/// C2H 10, ...) are not frames.
pub const TYPE_WIFI: u8 = 0;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct RxDesc {
    pub pkt_size: usize,
    pub pkt_type: u8,
    pub crc32_err: bool,
    pub icv_err: bool,
    pub hw_dec: bool,
    pub sw_dec: bool,
    pub ppdu_cnt: u8,
    /// Where the packet starts in the buffer: past the info word, the
    /// descriptor and the driver info.
    pub data: usize,
}

fn word(b: &[u8], i: usize) -> Option<u32> {
    Some(u32::from_le_bytes(b.get(4 * i..4 * i + 4)?.try_into().ok()?))
}

/// The descriptor at `buf[RXBD_INFO_SIZE..]`, or `None` if the buffer is
/// too short to hold it.
pub fn parse(buf: &[u8]) -> Option<RxDesc> {
    let d = buf.get(RXBD_INFO_SIZE..)?;
    let (w0, w1, w3) = (word(d, 0)?, word(d, 1)?, word(d, 3)?);
    let long = w0 & 0x8000_0000 != 0;
    let drv_info = ((w0 >> 28) & 0x7) as usize * 8;
    let rxd_len = if long { LONG } else { SHORT };
    if d.len() < rxd_len {
        return None;
    }
    Some(RxDesc {
        pkt_size: (w0 & 0x3fff) as usize,
        pkt_type: ((w0 >> 24) & 0xf) as u8,
        crc32_err: w3 & 0x200 != 0,
        icv_err: w3 & 0x400 != 0,
        hw_dec: w3 & 0x4 != 0,
        sw_dec: w3 & 0x2 != 0,
        ppdu_cnt: ((w1 >> 4) & 0x7) as u8,
        data: RXBD_INFO_SIZE + drv_info + rxd_len,
    })
}
