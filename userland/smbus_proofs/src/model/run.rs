// NONOS Operating System (AGPL-3.0-or-later)
//! What the modelled host does with a start, a kill and its buffer.

use super::host::Model;
use crate::regs::*;

impl Model {
    pub(super) fn control(&mut self, v: u8) {
        self.cnt = v;
        if v & CNT_KILL != 0 {
            self.sts = (self.sts & !STS_HOST_BUSY) | STS_FAILED;
            self.stuck = false;
        } else if v & CNT_START != 0 {
            self.sts |= STS_HOST_BUSY;
            self.pending = self.busy_reads;
        }
    }

    pub(super) fn next_block_byte(&mut self) -> u8 {
        let b = self.blk.get(self.idx).copied().unwrap_or(0);
        self.idx += 1;
        b
    }

    pub(super) fn put_block_byte(&mut self, v: u8) {
        if let Some(slot) = self.blk.get_mut(self.idx) {
            *slot = v;
        }
        self.idx += 1;
    }

    pub(super) fn finish(&mut self) {
        self.sts &= !STS_HOST_BUSY;
        let (addr, read) = (self.add >> 1, self.add & 1 != 0);
        let Some(i) = self.devices.iter().position(|d| d.addr == addr) else {
            self.result = Some(STS_DEV_ERR);
            return;
        };
        let (cmd, (d0, d1)) = (self.cmd as usize, (self.dat0, self.dat1));
        let block_ok = self.aux & AUX_E32B != 0;
        let d = &mut self.devices[i];
        match (self.cnt & 0x1C, read) {
            (CNT_BYTE, true) => (self.dat0, d.ptr) = (d.mem[d.ptr as usize], d.ptr.wrapping_add(1)),
            (CNT_BYTE, false) => d.ptr = cmd as u8,
            (CNT_BYTE_DATA, true) => self.dat0 = d.mem[cmd],
            (CNT_BYTE_DATA, false) => d.mem[cmd] = d0,
            (CNT_WORD_DATA, true) => (self.dat0, self.dat1) = (d.mem[cmd], d.mem[(cmd + 1) % 256]),
            (CNT_WORD_DATA, false) => (d.mem[cmd], d.mem[(cmd + 1) % 256]) = (d0, d1),
            (CNT_BLOCK_DATA, true) if block_ok => {
                self.dat0 = d.block[0];
                let n = d.block.len() - 1;
                self.blk[..n.min(32)].copy_from_slice(&d.block[1..1 + n.min(32)]);
            }
            (CNT_BLOCK_DATA, false) if block_ok => d.written = self.blk[..d0 as usize].to_vec(),
            (CNT_BLOCK_DATA, _) => self.force = Some(STS_FAILED | STS_INTR),
            _ => {}
        }
        self.result = Some(self.force.take().unwrap_or(STS_INTR));
    }
}
