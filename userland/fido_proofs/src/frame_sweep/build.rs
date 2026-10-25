// NONOS Operating System (AGPL-3.0-or-later)
//! A sound report stream on one channel (CTAP 2.x, 11.2.4, as recalled,
//! unverified): two to five messages of 0 to 7609 bytes, their lengths
//! spread over powers of two, each framed into 64-byte reports by the
//! driver's own framing; between messages, reports on other channels and
//! KEEPALIVEs, which leave every message whole.

use crate::ctaphid_parts::frame::{packets, Cid, CMD_CBOR, CMD_KEEPALIVE, MESSAGE_MAX, PACKET};

use super::rng::Rng;

pub const CID: Cid = [0x11, 0x22, 0x33, 0x44];

pub struct Stream {
    pub reports: Vec<Vec<u8>>,
    /// Each message's command and bytes, and where its reports start and end.
    pub messages: Vec<(u8, Vec<u8>, usize, usize)>,
}

pub fn keepalive(cid: Cid, status: u8) -> Vec<u8> {
    let mut p = vec![0u8; PACKET];
    p[..4].copy_from_slice(&cid);
    (p[4], p[5], p[6], p[7]) = (CMD_KEEPALIVE, 0, 1, status);
    p
}

pub fn stream(r: &mut Rng) -> Stream {
    let mut s = Stream { reports: Vec::new(), messages: Vec::new() };
    for m in 0..r.range(2, 5) {
        let k = r.range(0, 13);
        let len = (r.range(0, 1 << k) as usize).min(MESSAGE_MAX);
        let data: Vec<u8> = (0..len).map(|i| (i * 13 + m) as u8).collect();
        let cmd = r.one_of(&[CMD_CBOR as usize, 0x83, 0x81]) as u8;
        let start = s.reports.len();
        s.reports
            .extend(packets(CID, cmd, &data).unwrap().iter().map(|p: &[u8; PACKET]| p.to_vec()));
        s.messages.push((cmd, data, start, s.reports.len()));
        if r.range(0, 2) == 0 {
            let other = [0x55, 0x66, 0x77, r.range(0, 255) as u8];
            s.reports
                .extend(packets(other, CMD_CBOR, &[1, 2, 3]).unwrap().iter().map(|p| p.to_vec()));
        }
        if r.range(0, 2) == 0 {
            s.reports.push(keepalive(CID, r.one_of(&[1, 2]) as u8));
        }
    }
    s
}
