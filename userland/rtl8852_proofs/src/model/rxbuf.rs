// NONOS Operating System (AGPL-3.0-or-later)
//! A receive buffer as rtw89 reads it: the RX BD info word (first and last
//! segment, bytes written), a short RX descriptor (packet size and type),
//! then the packet.

pub const TYPE_WIFI: u8 = 0;
pub const TYPE_RELEASE: u8 = 7;
pub const TYPE_C2H: u8 = 10;

/// A buffer of packet type `kind` around `payload`.
pub fn wrap(kind: u8, payload: &[u8]) -> Vec<u8> {
    let len = 4 + 16 + payload.len();
    let info = (1u32 << 15) | (1 << 14) | len as u32;
    let w0 = payload.len() as u32 | (kind as u32) << 24;
    let mut b = info.to_le_bytes().to_vec();
    b.extend_from_slice(&w0.to_le_bytes());
    b.extend_from_slice(&[0; 12]);
    b.extend_from_slice(payload);
    b
}

/// A received MPDU, with a frame check sequence after it.
pub fn wifi(mpdu: &[u8]) -> Vec<u8> {
    wifi_with(mpdu, 0)
}

/// The same, descriptor word 3 carrying `w3` (`HW_DEC`, `ICV_ERR`).
pub fn wifi_with(mpdu: &[u8], w3: u32) -> Vec<u8> {
    let mut p = mpdu.to_vec();
    p.extend_from_slice(&[0xa5; 4]);
    let mut b = wrap(TYPE_WIFI, &p);
    b[16..20].copy_from_slice(&w3.to_le_bytes());
    b
}
