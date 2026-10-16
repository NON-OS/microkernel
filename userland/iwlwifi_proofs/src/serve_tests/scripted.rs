// NONOS Operating System (AGPL-3.0-or-later)
//! The modeled firmware's frames and scan completions the hunt proofs use.


use crate::gen3_model::Out;
use crate::join_rig::{beacon, RSNE_PSK};

// A beacon or probe response notification heard on `channel`.
pub(super) fn heard(frame: &[u8], channel: u8) -> Out {
    let mut p = vec![0u8; 64];
    p[0..2].copy_from_slice(&(frame.len() as u16).to_le_bytes());
    p[12] = 0x3;
    p[40] = 50;
    p[42] = channel;
    p.extend_from_slice(frame);
    Out { cmd: 0xC1, group: 0, seq: 0x8000, payload: p }
}

pub(super) fn completion(status: u8) -> Out {
    let mut p = vec![0u8; 16];
    p[6] = status;
    Out { cmd: 0x0F, group: 0, seq: 0x8000, payload: p }
}

pub(super) fn other_network() -> Vec<u8> {
    let mut b = beacon(&[&RSNE_PSK]);
    b[38] = b'H';
    b[10] ^= 0x10;
    b[16] ^= 0x10;
    b
}
