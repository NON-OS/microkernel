// NONOS Operating System (AGPL-3.0-or-later)

//! The MCU reading a command off ring 17: a unified one (option bit 1 in
//! byte 43) has its `cid` at 34 and body at 48; any other its `cid` at 36,
//! `set_query` at 38, `ext_cid` at 41, body at 64; both their `seq` at 39.
//! Each is kept and answered on the event ring, 0 until the RAM code runs
//! and 4 after, unless the chip is told to stay silent.

use super::chip::Chip;
use super::mcu::Seen;

pub fn le32(b: &[u8], at: usize) -> u32 {
    b.get(at..at + 4).map_or(0, |w| u32::from_le_bytes([w[0], w[1], w[2], w[3]]))
}

impl Chip {
    pub(super) fn command(&mut self, b: &[u8]) {
        if b.len() < 48 {
            return;
        }
        let uni = b[43] & 0x2 != 0;
        let (cid, body) = if uni {
            (u16::from_le_bytes([b[34], b[35]]), b[48..].to_vec())
        } else {
            (u16::from(b[36]), b.get(64..).unwrap_or(&[]).to_vec())
        };
        let (seq, set_query, ext) = (b[39], b[38], if uni { 0 } else { b[41] });
        self.mcu.seen.push(Seen { uni, cid, ext, seq, set_query, body: body.clone() });
        let to_data = uni && cid == 0x27 && self.faults.grant_on_data;
        let ring = if to_data {
            2
        } else if self.mcu.running {
            4
        } else {
            0
        };
        let out = if uni {
            self.unified(cid as u8, seq, &body)
        } else {
            self.legacy(cid as u8, seq, &body)
        };
        if let Some(e) = out.filter(|_| !self.faults.silent) {
            self.push_rx(ring, e);
        }
    }
}
