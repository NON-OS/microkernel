// NONOS Operating System (AGPL-3.0-or-later)
//! A security key as the host sees it (FIDO CTAP 2.x, 11.2, as recalled,
//! unverified): one HID interface with 64-byte interrupt IN and OUT
//! endpoints, and its answers in 64-byte reports.

fn ep(address: u8, max_packet: u16) -> Vec<u8> {
    vec![7, 0x05, address, 0x03, max_packet as u8, (max_packet >> 8) as u8, 5]
}

/// A configuration of one HID interface (class 03h) with `eps`.
pub fn config(eps: &[(u8, u16)]) -> Vec<u8> {
    let mut body = vec![9, 0x04, 0, 0, eps.len() as u8, 0x03, 0, 0, 0];
    body.extend([9, 0x21, 0x10, 0x01, 0, 1, 0x22, 34, 0]);
    for &(a, m) in eps {
        body.extend(ep(a, m));
    }
    let total = (9 + body.len()) as u16;
    let mut c = vec![9, 0x02, total as u8, (total >> 8) as u8, 1, 1, 0, 0x80, 50];
    c.extend(body);
    c
}

pub const KEY: [(u8, u16); 2] = [(0x81, 64), (0x01, 64)];

/// The first report of a message: channel, command, length, data.
pub fn first(cid: [u8; 4], cmd: u8, len: u16, data: &[u8]) -> Vec<u8> {
    let mut p = vec![0u8; 64];
    p[..4].copy_from_slice(&cid);
    p[4] = cmd;
    p[5..7].copy_from_slice(&len.to_be_bytes());
    p[7..7 + data.len()].copy_from_slice(data);
    p
}

/// A continuation report: channel, sequence, data.
pub fn more(cid: [u8; 4], seq: u8, data: &[u8]) -> Vec<u8> {
    let mut p = vec![0u8; 64];
    p[..4].copy_from_slice(&cid);
    p[4] = seq;
    p[5..5 + data.len()].copy_from_slice(data);
    p
}

/// INIT's answer: the nonce, the new channel, protocol 2, version 5.2.7,
/// and `caps`.
pub fn init_answer(nonce: [u8; 8], cid: [u8; 4], caps: u8) -> Vec<u8> {
    let mut d = nonce.to_vec();
    d.extend(cid);
    d.extend([2, 5, 2, 7, caps]);
    first([0xFF; 4], 0x86, 17, &d)
}
