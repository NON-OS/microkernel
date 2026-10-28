// NONOS Operating System (AGPL-3.0-or-later)

//! The modeled firmware's queues: SCD_QUEUE_CONFIG_CMD adds and removes
//! transmit queues, and a descriptor's buffers are read out of DMA memory.

use super::{reply, Mem, Model, Out, Seen};
use crate::gen3::region::Region;

/// The buffers the TFH descriptor at `tfd` (an offset into `mem`) names.
pub(super) fn buffers(mem: &Mem, tfd: usize) -> Vec<Vec<u8>> {
    let mut t = [0u8; 256];
    mem.read(tfd, &mut t);
    let n = u16::from_le_bytes([t[0], t[1]]) as usize;
    (0..n)
        .map(|i| {
            let at = 2 + i * 10;
            let len = u16::from_le_bytes([t[at], t[at + 1]]) as usize;
            let addr = u64::from_le_bytes(t[at + 2..at + 10].try_into().unwrap());
            let mut b = vec![0u8; len];
            mem.read((addr - mem.dev) as usize, &mut b);
            b
        })
        .collect()
}

impl Model {
    // SCD_QUEUE_CONFIG_CMD, answered by the transmit side when there is one.
    pub(super) fn queue_config(&self, c: &Seen) -> Option<Out> {
        if (c.group, c.cmd) != (0x5, 0x17) {
            return None;
        }
        let mut s = self.s.borrow_mut();
        let t = s.txq.as_mut()?;
        let word = |at: usize| u32::from_le_bytes(c.payload[at..at + 4].try_into().unwrap());
        let mut rsp = vec![0u8; t.queue_reply_len];
        if word(0) == 0 {
            let qid = t.next_qid;
            t.next_qid += 1;
            let bc = u64::from_le_bytes(c.payload[20..28].try_into().unwrap());
            let ring = u64::from_le_bytes(c.payload[28..36].try_into().unwrap());
            t.rings.insert(qid, (ring, bc));
            if rsp.len() >= 2 {
                rsp[0..2].copy_from_slice(&qid.to_le_bytes());
            }
            Some(reply(c, rsp))
        } else {
            // A removal: the queue is the one of this station and TID.
            Some(reply(c, vec![]))
        }
    }
}
