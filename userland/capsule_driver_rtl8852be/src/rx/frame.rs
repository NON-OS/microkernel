// NONOS Operating System (AGPL-3.0-or-later)

//! One filled RXQ buffer to one frame (`rtw89_pci_rxbd_deliver_skbs` and
//! `rtw89_core_rx`, Linux v6.12 rtw89 `pci.c`, `core.c`): an 802.11 frame
//! whose CRC and ICV checked out is copied out without its FCS (rtw89
//! declares RX_INCLUDES_FCS); PPDU status reports and firmware C2H
//! messages are not frames and are dropped, as is a frame split over
//! several buffers (the station's frames never exceed one 12 KiB buffer).

use super::desc::{parse, TYPE_WIFI};
use crate::ring::bd::{rx_info, RXBD_INFO_SIZE};

pub const FCS_LEN: usize = 4;
/// The shortest frame kept: an ACK's 10 bytes and its FCS.
pub const MIN_FRAME: usize = 10 + FCS_LEN;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Drop {
    Short,
    Split,
    NotFrame(u8),
    BadCrc,
    TooBig,
}

/// The frame's length in `out`, and whether the chip decrypted it.
pub fn take(buf: &[u8], out: &mut [u8]) -> Result<(usize, bool), Drop> {
    let head = buf.get(..RXBD_INFO_SIZE).ok_or(Drop::Short)?;
    let info = rx_info(u32::from_le_bytes(head.try_into().map_err(|_| Drop::Short)?));
    if !(info.fs && info.ls) {
        return Err(Drop::Split);
    }
    let written = (info.len as usize).min(buf.len());
    let d = parse(&buf[..written]).ok_or(Drop::Short)?;
    if d.pkt_type != TYPE_WIFI {
        return Err(Drop::NotFrame(d.pkt_type));
    }
    if d.crc32_err || d.icv_err {
        return Err(Drop::BadCrc);
    }
    if d.pkt_size < MIN_FRAME || d.data + d.pkt_size > written {
        return Err(Drop::Short);
    }
    let n = d.pkt_size - FCS_LEN;
    let dst = out.get_mut(..n).ok_or(Drop::TooBig)?;
    dst.copy_from_slice(&buf[d.data..d.data + n]);
    Ok((n, d.hw_dec && !d.sw_dec))
}
