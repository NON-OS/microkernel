// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! The adapter as `nonos_scsi` drives it: one request at a time through the
//! request ring, its completion read back from the completion ring.

use core::ptr::{read_volatile, write_volatile};
use core::sync::atomic::{fence, Ordering};

use nonos_scsi::{Addr, Dir, Fault, Hba, Outcome, SENSE_MAX};

use crate::adapter;
use crate::constants::layout::{
    CMP_LOG2, CMP_RING_OFF, DATA_BYTES, DATA_OFF, REQ_LOG2, REQ_RING_OFF, SENSE_BYTES, SENSE_OFF,
    STATE_OFF,
};
use crate::constants::regs::{
    CDB_MAX, CMP_BYTES, CMP_CONTEXT, CMP_DATA_LEN, CMP_HOST_STATUS,
    CMP_SCSI_STATUS, CMP_SENSE_LEN, FLAG_DIR_NONE, FLAG_DIR_TODEVICE, FLAG_DIR_TOHOST,
    REG_KICK_NON_RW_IO, REG_KICK_RW_IO, REQ_BUS, REQ_BYTES, REQ_CDB, REQ_CDB_LEN, REQ_CONTEXT,
    REQ_DATA_ADDR, REQ_DATA_LEN, REQ_FLAGS, REQ_LUN, REQ_SENSE_ADDR, REQ_SENSE_LEN, REQ_TARGET,
    RS_CMP_CONS, RS_CMP_PROD, RS_REQ_PROD,
};
use crate::constants::timing::COMMAND_MS;
use crate::regs::Regs;
use crate::wait;

pub struct PvHba<R: Regs> {
    pub regs: R,
    /// The grant: where the driver reaches it and where the adapter does.
    pub va: u64,
    pub dev: u64,
    next_id: u64,
    /// The adapter did not come back from a reset; nothing more is sent.
    pub dead: bool,
}

/// READ and WRITE in every length: the adapter wants them on their own
/// doorbell.
fn is_rw(op: u8) -> bool {
    matches!(op, 0x08 | 0x0A | 0x28 | 0x2A | 0xA8 | 0xAA | 0x88 | 0x8A)
}

impl<R: Regs> PvHba<R> {
    pub fn new(regs: R, va: u64, dev: u64) -> Self {
        Self { regs, va, dev, next_id: 0, dead: false }
    }

    fn at<T>(&self, off: usize) -> *mut T {
        (self.va as usize + off) as *mut T
    }

    fn state(&self, off: usize) -> u32 {
        // SAFETY: a word of the rings state page in the grant, mapped for
        // the driver's life.
        unsafe { read_volatile(self.at::<u32>(STATE_OFF + off)) }
    }

    fn set_state(&self, off: usize, v: u32) {
        // SAFETY: as above.
        unsafe { write_volatile(self.at::<u32>(STATE_OFF + off), v) }
    }

    fn put<T>(&self, off: usize, v: T) {
        // SAFETY: `off` lies in the grant's ring pages (callers add a field
        // offset inside one descriptor slot).
        unsafe { write_volatile(self.at::<T>(off), v) }
    }

    fn get<T: Copy>(&self, off: usize) -> T {
        // SAFETY: as above.
        unsafe { read_volatile(self.at::<T>(off)) }
    }

    /// A request never completed: reset the adapter, which drops it, and
    /// hand it fresh rings. An adapter that does not take them is given up
    /// on.
    fn recover(&mut self) {
        if adapter::start(&self.regs, self.va, self.dev).is_err() {
            self.dead = true;
        }
    }
}

impl<R: Regs> Hba for PvHba<R> {
    fn buffer(&mut self) -> &mut [u8] {
        // SAFETY: the data buffer is DATA_BYTES of the grant after its ring
        // pages, mapped for the driver's life; `&mut self` keeps it unique.
        unsafe { core::slice::from_raw_parts_mut(self.at::<u8>(DATA_OFF), DATA_BYTES) }
    }

    fn exec(&mut self, at: Addr, cdb: &[u8], dir: Dir, len: usize) -> Result<Outcome, Fault> {
        if self.dead || cdb.is_empty() || cdb.len() > CDB_MAX || len > DATA_BYTES || at.lun > 0xFF {
            return Err(Fault::Failed);
        }
        self.next_id = self.next_id.wrapping_add(1);
        let id = self.next_id;
        let prod = self.state(RS_REQ_PROD);
        let d = REQ_RING_OFF + (prod & ((1 << REQ_LOG2) - 1)) as usize * REQ_BYTES;
        // SAFETY: one descriptor slot of the request ring page.
        unsafe { core::ptr::write_bytes(self.at::<u8>(d), 0, REQ_BYTES) };
        if matches!(dir, Dir::In) && !is_rw(cdb[0]) {
            // SAFETY: `len` is within the data buffer (checked above).
            unsafe { core::ptr::write_bytes(self.at::<u8>(DATA_OFF), 0, len) };
        }
        let flags = match dir {
            Dir::None => FLAG_DIR_NONE,
            Dir::In => FLAG_DIR_TOHOST,
            Dir::Out => FLAG_DIR_TODEVICE,
        };
        self.put(d + REQ_CONTEXT, id);
        self.put(d + REQ_DATA_ADDR, if len == 0 { 0 } else { self.dev + DATA_OFF as u64 });
        self.put(d + REQ_DATA_LEN, len as u64);
        self.put(d + REQ_SENSE_ADDR, self.dev + SENSE_OFF as u64);
        self.put(d + REQ_SENSE_LEN, SENSE_BYTES);
        self.put(d + REQ_FLAGS, flags);
        for (i, &b) in cdb.iter().enumerate() {
            self.put(d + REQ_CDB + i, b);
        }
        self.put(d + REQ_CDB_LEN, cdb.len() as u8);
        // A single-level LUN in byte 1, on bus 0.
        self.put(d + REQ_LUN + 1, at.lun as u8);
        self.put(d + REQ_BUS, 0u8);
        self.put(d + REQ_TARGET, at.target as u8);
        // The descriptor must be visible before the index that publishes it.
        fence(Ordering::Release);
        self.set_state(RS_REQ_PROD, prod.wrapping_add(1));
        let kick = if is_rw(cdb[0]) { REG_KICK_RW_IO } else { REG_KICK_NON_RW_IO };
        self.regs.w32(kick, 0);
        let cons = self.state(RS_CMP_CONS);
        if !wait::until(COMMAND_MS, || self.state(RS_CMP_PROD) != cons) {
            self.recover();
            return Err(Fault::Timeout);
        }
        // The completion is the driver's once the index that covers it is
        // seen.
        fence(Ordering::Acquire);
        let c = CMP_RING_OFF + (cons & ((1 << CMP_LOG2) - 1)) as usize * CMP_BYTES;
        let context: u64 = self.get(c + CMP_CONTEXT);
        let moved: u64 = self.get(c + CMP_DATA_LEN);
        let sense_len: u32 = self.get(c + CMP_SENSE_LEN);
        let host: u16 = self.get(c + CMP_HOST_STATUS);
        let scsi: u16 = self.get(c + CMP_SCSI_STATUS);
        self.set_state(RS_CMP_CONS, cons.wrapping_add(1));
        if context != id {
            return Err(Fault::Failed);
        }
        let residual = crate::moved::completed(host, len, moved, is_rw(cdb[0]))?;
        let mut o = Outcome::good();
        o.status = scsi as u8;
        o.residual = residual;
        o.sense_len = (sense_len as usize).min(SENSE_BYTES as usize).min(SENSE_MAX);
        for i in 0..o.sense_len {
            o.sense[i] = self.get(SENSE_OFF + i);
        }
        Ok(o)
    }
}
