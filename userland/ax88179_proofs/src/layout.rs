// NONOS Operating System (AGPL-3.0-or-later)
//! Bytes built to the layouts the driver reads. The configurations are
//! built to the interface products[] matches, not dumped from a device;
//! receive transfers follow the layout ax88179_rx_fixup's comment gives.

use crate::ax::rx::RXHDR_DROP_ERR;

/// One configuration: interface 0, class ff/ff/00, an interrupt IN, then
/// bulk IN 0x82 and bulk OUT 0x03 of 1024 bytes with SuperSpeed
/// companions (burst 3).
pub const CONFIG: [u8; 51] = [
    9, 2, 51, 0, 1, 1, 0, 0x80, 0x32, // configuration 1
    9, 4, 0, 0, 3, 0xff, 0xff, 0, 0, // interface 0
    7, 5, 0x81, 3, 8, 0, 11, // interrupt IN
    7, 5, 0x82, 2, 0x00, 0x04, 0, // bulk IN
    6, 0x30, 3, 0, 0, 0, // its companion
    7, 5, 0x03, 2, 0x00, 0x04, 0, // bulk OUT
    6, 0x30, 3, 0, 0, 0, // its companion
];

/// A configuration whose first interface is CDC (communications, ECM).
pub const CLASS_CONFIG: [u8; 18] = [9, 2, 18, 0, 1, 2, 0, 0x80, 0x32, 9, 4, 0, 0, 1, 2, 6, 0, 0];

/// A bulk IN as ax88179_rx_fixup describes it: each packet as the 2-byte
/// alignment header and the frame, padded to 8; then the per-packet
/// headers (length with the alignment header, in the high half, and the
/// flags), each followed by a dummy header of length 0 and DROP_ERR when
/// `dummies`; 4 bytes so the last word ends on 8; then count and offset.
pub fn rx_transfer(packets: &[(&[u8], u32)], dummies: bool) -> Vec<u8> {
    let (mut out, mut hdrs) = (Vec::new(), Vec::new());
    for (frame, flags) in packets {
        out.extend([0u8, 0]);
        out.extend_from_slice(frame);
        while out.len() % 8 != 0 {
            out.push(0);
        }
        hdrs.push(((frame.len() as u32 + 2) << 16) | flags);
        if dummies {
            hdrs.push(RXHDR_DROP_ERR);
        }
    }
    let hdr_off = out.len() as u32;
    hdrs.iter().for_each(|h| out.extend(h.to_le_bytes()));
    if (out.len() + 4) % 8 != 0 {
        out.extend([0u8; 4]);
    }
    out.extend((hdrs.len() as u32 | hdr_off << 16).to_le_bytes());
    out
}

/// A frame of `len` bytes whose every byte is `fill`.
pub fn frame(len: usize, fill: u8) -> Vec<u8> {
    vec![fill; len]
}
