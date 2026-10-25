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

//! The IOC as `nonos_scsi` drives it: one SCSI IO request at a time posted
//! to the request FIFO, its completion read from the reply FIFO: the
//! context alone when it went well, a reply frame when there is more to say.

use core::ptr::{read_volatile, write_bytes, write_volatile};
use core::sync::atomic::{fence, Ordering};

use nonos_scsi::{Addr, Dir, Fault, Hba, Outcome, SENSE_MAX};

use crate::constants::layout::{
    DATA_BYTES, DATA_OFF, REPLY_BYTES, REPLY_FRAMES, REPLY_OFF, REQUEST_BYTES, REQUEST_OFF,
    SENSE_BYTES, SENSE_OFF,
};
use crate::constants::msg::{
    CONTROL_NO_DATA, CONTROL_READ, CONTROL_WRITE, IOR_SCSI_STATE, IOR_SCSI_STATUS, IOR_SENSE_COUNT,
    IOR_TRANSFER_COUNT, IO_BUS, IO_CDB, IO_CDB_LEN, IO_CDB_MAX, IO_CONTROL, IO_DATA_LEN, IO_LUN,
    IO_SENSE_ADDR, IO_SENSE_LEN, IO_SGL, IO_TARGET, MSG_CONTEXT, MSG_FUNCTION, REPLY_IOC_STATUS,
    SGE_HOST_TO_IOC, SGE_SIMPLE_LAST,
};
use crate::constants::regs::{
    ADDRESS_REPLY, FIFO_EMPTY, FUNCTION_SCSI_IO_REQUEST, IOCSTATUS_BUSY, IOCSTATUS_MASK,
    IOCSTATUS_SCSI_DATA_UNDERRUN, IOCSTATUS_SCSI_DEVICE_NOT_THERE, IOCSTATUS_SCSI_EXT_TERMINATED,
    IOCSTATUS_SCSI_INVALID_BUS, IOCSTATUS_SCSI_INVALID_TARGETID, IOCSTATUS_SCSI_IOC_TERMINATED,
    IOCSTATUS_SCSI_RECOVERED_ERROR, IOCSTATUS_SCSI_TASK_TERMINATED, IOCSTATUS_SUCCESS,
    REG_REPLY_FIFO, REG_REQUEST_FIFO, SCSI_STATE_AUTOSENSE_VALID,
};
use crate::constants::timing::REQUEST_MS;
use crate::ioc;
use crate::regs::Regs;
use crate::wait;

/// SCSI request contexts count up from one and stay below the driver's own
/// messages' and clear of the context reply's type bits.
const CONTEXT_MAX: u32 = 0x0FFF_FFFF;

pub struct MptHba<R: Regs> {
    pub regs: R,
    /// The grant: where the driver reaches it and where the IOC does, below
    /// 4 GiB.
    pub va: u64,
    pub dev: u64,
    /// Targets the IOC addresses on its bus.
    pub max_devices: u8,
    context: u32,
    /// The IOC did not come back from a reset; nothing more is sent.
    pub dead: bool,
}

impl<R: Regs> MptHba<R> {
    pub fn new(regs: R, va: u64, dev: u64, max_devices: u8) -> Self {
        Self { regs, va, dev, max_devices, context: 0, dead: false }
    }

    fn put<T>(&self, off: usize, v: T) {
        // SAFETY: `off` is a field of the request frame inside the grant,
        // which stays mapped while the driver lives.
        unsafe { write_volatile((self.va as usize + off) as *mut T, v) }
    }

    fn get<T: Copy>(&self, off: usize) -> T {
        // SAFETY: as above, for a reply frame or the sense buffer.
        unsafe { read_volatile((self.va as usize + off) as *const T) }
    }

    fn bus(&self, off: usize) -> u32 {
        (self.dev + off as u64) as u32
    }

    /// A request never completed: a message unit reset drops it, and the IOC
    /// is brought up again. One that does not come up is given up on.
    fn recover(&mut self) {
        if ioc::start(&self.regs, self.va, self.dev).is_err() {
            self.dead = true;
        }
    }
}

impl<R: Regs> Hba for MptHba<R> {
    fn buffer(&mut self) -> &mut [u8] {
        // SAFETY: the data buffer is DATA_BYTES of the grant after its first
        // page, mapped for the driver's life; `&mut self` keeps it unique.
        unsafe {
            core::slice::from_raw_parts_mut((self.va as usize + DATA_OFF) as *mut u8, DATA_BYTES)
        }
    }

    fn exec(&mut self, at: Addr, cdb: &[u8], dir: Dir, len: usize) -> Result<Outcome, Fault> {
        if self.dead || cdb.is_empty() || cdb.len() > IO_CDB_MAX || len > DATA_BYTES {
            return Err(Fault::Failed);
        }
        let devices = if self.max_devices == 0 { 256 } else { self.max_devices as u16 };
        if at.target >= devices || at.lun > 0xFF {
            return Err(Fault::NoTarget);
        }
        self.context = self.context % CONTEXT_MAX + 1;
        let context = self.context;
        // SAFETY: the request frame, inside the grant as above.
        unsafe { write_bytes((self.va as usize + REQUEST_OFF) as *mut u8, 0, REQUEST_BYTES) };
        let f = REQUEST_OFF;
        self.put(f + IO_TARGET, at.target as u8);
        self.put(f + IO_BUS, 0u8);
        self.put(f + MSG_FUNCTION, FUNCTION_SCSI_IO_REQUEST);
        self.put(f + IO_CDB_LEN, cdb.len() as u8);
        self.put(f + IO_SENSE_LEN, SENSE_BYTES);
        self.put(f + MSG_CONTEXT, context.to_le());
        // A single-level LUN in byte 1.
        self.put(f + IO_LUN + 1, at.lun as u8);
        let (control, sge_dir) = match dir {
            Dir::None => (CONTROL_NO_DATA, 0),
            Dir::In => (CONTROL_READ, 0),
            Dir::Out => (CONTROL_WRITE, SGE_HOST_TO_IOC),
        };
        self.put(f + IO_CONTROL, control.to_le());
        for (i, &b) in cdb.iter().enumerate() {
            self.put(f + IO_CDB + i, b);
        }
        self.put(f + IO_DATA_LEN, (len as u32).to_le());
        self.put(f + IO_SENSE_ADDR, self.bus(SENSE_OFF).to_le());
        self.put(f + IO_SGL, (SGE_SIMPLE_LAST | sge_dir | len as u32).to_le());
        self.put(f + IO_SGL + 4, if len == 0 { 0 } else { self.bus(DATA_OFF).to_le() });
        // The frame must be visible before the FIFO write that posts it.
        fence(Ordering::Release);
        self.regs.w32(REG_REQUEST_FIFO, self.bus(REQUEST_OFF));
        let mut v = FIFO_EMPTY;
        let regs = &self.regs;
        if !wait::until(REQUEST_MS, || {
            v = regs.r32(REG_REPLY_FIFO);
            v != FIFO_EMPTY
        }) {
            self.recover();
            return Err(Fault::Timeout);
        }
        fence(Ordering::Acquire);
        if v & ADDRESS_REPLY == 0 {
            // A context reply: the request went well, all of it.
            return if v == context { Ok(Outcome::good()) } else { Err(Fault::Failed) };
        }
        let frame = v << 1;
        let off = frame.wrapping_sub(self.bus(REPLY_OFF)) as usize;
        if off >= REPLY_FRAMES * REPLY_BYTES || !off.is_multiple_of(REPLY_BYTES) {
            return Err(Fault::Failed);
        }
        let r = REPLY_OFF + off;
        let replied: u32 = u32::from_le(self.get(r + MSG_CONTEXT));
        let iocstatus = u16::from_le(self.get(r + REPLY_IOC_STATUS)) & IOCSTATUS_MASK;
        let scsi: u8 = self.get(r + IOR_SCSI_STATUS);
        let state: u8 = self.get(r + IOR_SCSI_STATE);
        let moved = u32::from_le(self.get(r + IOR_TRANSFER_COUNT));
        let sense_count = u32::from_le(self.get(r + IOR_SENSE_COUNT));
        // Read, the frame goes back to the reply free FIFO.
        self.regs.w32(REG_REPLY_FIFO, frame);
        if replied != context {
            return Err(Fault::Failed);
        }
        match iocstatus {
            IOCSTATUS_SUCCESS | IOCSTATUS_SCSI_RECOVERED_ERROR | IOCSTATUS_SCSI_DATA_UNDERRUN => {}
            IOCSTATUS_SCSI_INVALID_BUS
            | IOCSTATUS_SCSI_INVALID_TARGETID
            | IOCSTATUS_SCSI_DEVICE_NOT_THERE => return Err(Fault::NoTarget),
            IOCSTATUS_BUSY
            | IOCSTATUS_SCSI_TASK_TERMINATED
            | IOCSTATUS_SCSI_IOC_TERMINATED
            | IOCSTATUS_SCSI_EXT_TERMINATED => return Err(Fault::Reset),
            _ => return Err(Fault::Failed),
        }
        let mut o = Outcome::good();
        o.status = scsi;
        o.residual = (len as u32).saturating_sub(moved);
        if state & SCSI_STATE_AUTOSENSE_VALID != 0 {
            o.sense_len = (sense_count as usize).min(SENSE_BYTES as usize).min(SENSE_MAX);
            for i in 0..o.sense_len {
                o.sense[i] = self.get(SENSE_OFF + i);
            }
        }
        Ok(o)
    }
}
