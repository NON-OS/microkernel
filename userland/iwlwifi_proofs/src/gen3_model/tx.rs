// NONOS Operating System (AGPL-3.0-or-later)

//! The modeled firmware's transmit side: queue adds and removals, and each
//! frame decoded from its ring, checked against its byte count entry,
//! answered and handed to the scripted access point.

use super::queues::buffers;
use super::{Model, Out, Sent};
use crate::gen3::region::Region;

impl Model {
    // Decode the frame queued on `qid` whose doorbell was just rung.
    pub(super) fn tx_doorbell(&self, qid: u16, wptr: u32) {
        let slot = (wptr.wrapping_sub(1) as usize) & 127;
        let (mem, ring, bc) = {
            let s = self.s.borrow();
            let Some(t) = s.txq.as_ref() else { return };
            let Some(&(ring, bc)) = t.rings.get(&qid) else { return };
            (t.mem.clone(), ring, bc)
        };
        let tbs = buffers(&mem, (ring - mem.dev) as usize + slot * 256);
        let cmd: Vec<u8> = tbs.iter().take(2).flatten().copied().collect();
        let body: Vec<u8> = tbs.get(2).cloned().unwrap_or_default();
        let frame_len = u16::from_le_bytes([cmd[4], cmd[5]]) as usize;
        let hdr_len = frame_len - body.len();
        let mut frame = cmd[32..32 + hdr_len].to_vec();
        frame.extend_from_slice(&body);
        let mut entry = [0u8; 2];
        mem.read((bc - mem.dev) as usize + slot * 2, &mut entry);
        let sent = Sent {
            queue: qid,
            frame: frame.clone(),
            flags: u16::from_le_bytes([cmd[6], cmd[7]]),
            rate: u32::from_le_bytes(cmd[20..24].try_into().unwrap()),
        };
        let (air, respond) = {
            let mut s = self.s.borrow_mut();
            let t = s.txq.as_mut().unwrap();
            if u16::from_le_bytes(entry) & 0x3FFF != frame_len as u16 || cmd[0] != 0x1C {
                t.bc_mismatch += 1;
            }
            t.sent.push(sent);
            (t.air.take(), !t.no_tx_response)
        };
        if respond {
            let mut p = vec![0u8; 48];
            p[0] = 1;
            p[36..38].copy_from_slice(&qid.to_le_bytes());
            p[40] = 1;
            p[44..46].copy_from_slice(&(wptr as u16).to_le_bytes());
            let seq = (qid & 0x1F) << 8 | slot as u16;
            self.queue(Out { cmd: 0x1C, group: 0, seq, payload: p });
        }
        if let Some(mut air) = air {
            let outs = air(&frame);
            if let Some(t) = self.s.borrow_mut().txq.as_mut() {
                t.air = Some(air);
            }
            for o in outs {
                self.queue(o);
            }
        }
    }
}
