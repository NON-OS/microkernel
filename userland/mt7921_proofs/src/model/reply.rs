// NONOS Operating System (AGPL-3.0-or-later)

//! Events as the MCU writes them: a hardware RXD of packet type
//! `PKT_TYPE_RX_EVENT`, the connac2 event header (`eid` at 28, `seq` at
//! 29, `option` at 30, `ext_eid` at 32), then the body at 36.

/// An event with `eid`, `seq`, `option`, the byte at 32, and `body`.
pub fn event(eid: u8, seq: u8, option: u8, at32: u8, body: &[u8]) -> Vec<u8> {
    let total = 36 + body.len();
    let mut e = vec![0u8; total];
    e[0..4].copy_from_slice(&(total as u32 | 7 << 27).to_le_bytes());
    e[24..26].copy_from_slice(&((total - 24) as u16).to_le_bytes());
    (e[28], e[29], e[30], e[32]) = (eid, seq, option, at32);
    e[36..].copy_from_slice(body);
    e
}

/// The reply that answers in one byte (the semaphore, the patch finish).
pub fn byte(seq: u8, status: u8) -> Vec<u8> {
    event(0x01, seq, 0, status, &[])
}

/// A generic reply (`MCU_EVENT_GENERIC`, or `0xed` for an extended one).
pub fn generic(seq: u8, eid: u8) -> Vec<u8> {
    event(eid, seq, 0, 0, &[0; 4])
}

/// A unified command's reply: its `cid` and a status word.
pub fn uni(seq: u8, cid: u8, status: u32) -> Vec<u8> {
    let s = status.to_le_bytes();
    event(cid, seq, 0x02, 0, &[cid, 0, 0, 0, s[0], s[1], s[2], s[3]])
}

/// The ROC grant, unasked (`MCU_UNI_EVENT_ROC`, option unsolicited):
/// four bytes, then the grant TLV with the token and channel.
pub fn grant(token: u8, channel: u8) -> Vec<u8> {
    let mut b = vec![0u8; 4 + 20];
    b[6] = 20;
    (b[9], b[11]) = (token, channel);
    event(0x27, 0, 0x06, 0, &b)
}

/// `GET_NIC_CAPAB`'s reply: a factory address record, which the driver
/// must never read, and the PHY record (two streams, both bands).
pub fn capab(factory: [u8; 6]) -> Vec<u8> {
    let mut b = vec![2, 0, 0, 0];
    b.extend_from_slice(&7u32.to_le_bytes());
    b.extend_from_slice(&6u32.to_le_bytes());
    b.extend_from_slice(&factory);
    b.extend_from_slice(&8u32.to_le_bytes());
    b.extend_from_slice(&12u32.to_le_bytes());
    b.extend_from_slice(&[1, 1, 1, 0, 2, 0, 0, 0, 0, 0, 3, 1]);
    b
}
