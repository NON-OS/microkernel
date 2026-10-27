// NONOS Operating System (AGPL-3.0-or-later)

//! The unified commands as the MCU answers them. A channel request
//! (`UNI_ROC_ACQUIRE`, tag 0) moves the radio to its channel and is granted
//! unasked with its token; an abort (tag 1) leaves the channel. A BSS's
//! RLM TLV (tag 2) gives the BSS's channel to its context; its basic TLV
//! disconnected (state 1) takes it away. Every device, BSS and station
//! command is answered with the status the faults name.

use super::chip::Chip;
use super::reply;

impl Chip {
    pub(super) fn unified(&mut self, cid: u8, seq: u8, body: &[u8]) -> Option<Vec<u8>> {
        let tag = body.get(4..6).map_or(u16::MAX, |t| u16::from_le_bytes([t[0], t[1]]));
        match (cid, tag) {
            (0x27, 0) => {
                self.mcu.roc = body.get(10).copied();
                self.mcu.grants += 1;
                let (stale, f) = (u8::from(self.faults.stale_grant), &self.faults);
                let token = if f.tokenless_grant { 0 } else { body[9].wrapping_add(stale) };
                (!f.no_grant).then(|| reply::grant(token, body[10].wrapping_add(stale)))
            }
            (0x27, _) => {
                self.mcu.roc = None;
                None
            }
            (0x02, 2) => {
                self.mcu.bss = body.get(8).copied();
                Some(reply::uni(seq, cid, self.faults.uni_status))
            }
            (0x02, 0) if body.get(16) == Some(&1) => {
                self.mcu.bss = None;
                Some(reply::uni(seq, cid, self.faults.uni_status))
            }
            _ => Some(reply::uni(seq, cid, self.faults.uni_status)),
        }
    }
}
