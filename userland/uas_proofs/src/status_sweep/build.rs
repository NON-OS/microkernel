// NONOS Operating System (AGPL-3.0-or-later)
//! A sound exchange for one command (UAS 1.0, as recalled, unverified): its
//! tag, its data's way, whether the device has streams, and the IUs its
//! status pipe gives: READ or WRITE READY first on a USB 2 device with data
//! to move, then the Sense IU, a 16-byte header and 0 to 18 bytes of sense,
//! its length in bytes 14 and 15.

use nonos_uas::DataDir;

use super::rng::Rng;

pub const SENSE: u8 = 0x03;
pub const RESPONSE: u8 = 0x04;
pub const READ_READY: u8 = 0x06;
pub const WRITE_READY: u8 = 0x07;

pub struct Exchange {
    pub tag: u16,
    pub dir: DataDir,
    pub streams: bool,
    pub ius: Vec<Vec<u8>>,
    /// The status and sense the command must end with.
    pub status: u8,
    pub sense: Vec<u8>,
}

/// An IU of `id` under `tag`, `len` bytes, the rest zero.
pub fn iu(id: u8, tag: u16, len: usize) -> Vec<u8> {
    let mut b = vec![0u8; len.max(4)];
    b[0] = id;
    b[2..4].copy_from_slice(&tag.to_be_bytes());
    b.truncate(len);
    b
}

pub fn sense_iu(tag: u16, status: u8, sense: &[u8], extra: usize) -> Vec<u8> {
    let mut b = iu(SENSE, tag, 16);
    b[6] = status;
    b[14..16].copy_from_slice(&(sense.len() as u16).to_be_bytes());
    b.extend_from_slice(sense);
    b.extend(core::iter::repeat_n(0xEE, extra));
    b
}

pub fn exchange(r: &mut Rng) -> Exchange {
    let tag = r.range(1, 0xFFFF) as u16;
    let dir = [DataDir::None, DataDir::In, DataDir::Out][r.range(0, 2)];
    let streams = r.range(0, 1) == 1;
    let status = r.one_of(&[0, 0, 2]) as u8;
    let sense: Vec<u8> = (0..r.range(0, 18)).map(|i| 0x70 + i as u8).collect();
    let mut ius = Vec::new();
    if !streams && dir != DataDir::None {
        ius.push(iu(if dir == DataDir::In { READ_READY } else { WRITE_READY }, tag, 4));
    }
    ius.push(sense_iu(tag, status, &sense, r.one_of(&[0, 0, 4])));
    Exchange { tag, dir, streams, ius, status, sense }
}
