// NONOS Operating System (AGPL-3.0-or-later)

//! A received frame as the chip writes it on ring 2: a normal RXD, group 1
//! when RXD1 announces it (the packet number of a frame the chip opened,
//! PN0 first), group 3, whose first chain carries the signal as an RCPI,
//! then the frame.

/// The buffer for `frame` heard at `rssi`, RXD1 carrying `d1` besides
/// group 3; group 1 holds `pn` when `d1` announces it.
pub fn rx_buffer_with(frame: &[u8], rssi: i8, d1: u32, pn: u64) -> Vec<u8> {
    let rcpi = (2 * i32::from(rssi) + 220) as u32 & 0xff;
    let g1 = d1 & 1 << 11 != 0;
    let len = (32 + 16 * usize::from(g1) + frame.len()) as u32;
    let mut out: Vec<u8> =
        [len | 2 << 27, d1 | 1 << 13, 0, 0, 0, 0].iter().flat_map(|w| w.to_le_bytes()).collect();
    if g1 {
        out.extend_from_slice(&pn.to_le_bytes());
        out.extend_from_slice(&[0; 8]);
    }
    out.extend_from_slice(&[0, 0, 0, 0]);
    out.extend_from_slice(&(rcpi | rcpi << 8).to_le_bytes());
    out.extend_from_slice(frame);
    out
}

/// A frame heard in the clear: RXD, group 3, the frame.
pub fn rx_buffer(frame: &[u8], rssi: i8) -> Vec<u8> {
    rx_buffer_with(frame, rssi, 0, 0)
}
