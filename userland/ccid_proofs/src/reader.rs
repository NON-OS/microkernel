// NONOS Operating System (AGPL-3.0-or-later)
//! A reader's configuration (USB CCID 1.1, 5.1, as recalled, unverified):
//! one interface of class 0Bh, its 54-byte class descriptor, a bulk pair
//! and an interrupt IN for slot changes.

pub fn ep(address: u8, attrs: u8, max_packet: u16, interval: u8) -> Vec<u8> {
    vec![7, 0x05, address, attrs, max_packet as u8, (max_packet >> 8) as u8, interval]
}

/// The class descriptor of a reader with slots 0 to `max_slot`, these
/// dwFeatures and this dwMaxCCIDMessageLength.
pub fn class(max_slot: u8, features: u32, max_message: u32) -> Vec<u8> {
    let mut d = vec![0u8; 54];
    (d[0], d[1], d[2], d[3], d[4], d[5]) = (54, 0x21, 0x10, 0x01, max_slot, 0x07);
    d[6] = 0x03; // T=0 and T=1
    d[40..44].copy_from_slice(&features.to_le_bytes());
    d[44..48].copy_from_slice(&max_message.to_le_bytes());
    d
}

/// The configuration: the interface, then `class` (empty for none), then
/// the bulk pair and, when `notify`, the interrupt endpoint.
pub fn config(class: &[u8], notify: bool) -> Vec<u8> {
    let endpoints = 2 + notify as u8;
    let mut body = vec![9, 0x04, 0, 0, endpoints, 0x0B, 0, 0, 0];
    body.extend(class);
    body.extend(ep(0x81, 0x02, 64, 0));
    body.extend(ep(0x02, 0x02, 64, 0));
    if notify {
        body.extend(ep(0x83, 0x03, 8, 16));
    }
    let total = (9 + body.len()) as u16;
    let mut c = vec![9, 0x02, total as u8, (total >> 8) as u8, 1, 1, 0, 0x80, 50];
    c.extend(body);
    c
}

/// An APDU-level reader of short and extended APDUs, one slot.
pub const EXTENDED: u32 = 0x0004_0000 | 0x0000_00BA;
pub const SHORT: u32 = 0x0002_0000;
/// A TPDU-level reader: the host would have to speak T=1.
pub const TPDU: u32 = 0x0001_0000 | 0x0000_00BA;

/// An RDR_to_PC message: `kind`, `slot`, `seq`, bStatus, bError, `data`.
pub fn answer(kind: u8, slot: u8, seq: u8, status: u8, error: u8, data: &[u8]) -> Vec<u8> {
    let mut m = vec![kind];
    m.extend((data.len() as u32).to_le_bytes());
    m.extend([slot, seq, status, error, 0]);
    m.extend(data);
    m
}
