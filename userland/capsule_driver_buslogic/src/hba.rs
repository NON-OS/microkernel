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

//! The adapter as `nonos_scsi` drives it: one command control block at a
//! time through the outgoing mailbox, its completion read from the incoming
//! one.

use core::ptr::{read_volatile, write_bytes, write_volatile};
use core::sync::atomic::{fence, Ordering};

use nonos_scsi::{Addr, Dir, Fault, Hba, Outcome, SENSE_MAX};

use crate::adapter;
use crate::constants::layout::{
    CCB_OFF, DATA_BYTES, DATA_OFF, MBI_OFF, MBO_OFF, SENSE_BYTES, SENSE_OFF,
};
use crate::constants::regs::{
    ACTION_START, BT_BUS_FREE, BT_DATA_RUN, BT_HA_RESET, BT_LINKED_COMPLETE,
    BT_LINKED_COMPLETE_FLAG, BT_OTHER_RESET, BT_SELECTION_TIMEOUT, BT_SUCCESS, CCB_BYTES, CCB_CDB,
    CCB_CDB_LEN, CCB_DATA, CCB_DATA_LEN, CCB_DIRECTION, CCB_LUN, CCB_OPCODE, CCB_SENSE,
    CCB_SENSE_LEN, CCB_TARGET, CDB_MAX, COMP_ABORTED, COMP_ABORTED_NOT_FOUND, COMP_DONE,
    COMP_ERROR, COMP_FREE, CTL_INTERRUPT_RESET, DIR_IN, DIR_NONE, DIR_OUT, MAX_LUN, MAX_TARGET,
    MBI_COMPLETION, MBI_HOST_STATUS, MBI_TARGET_STATUS, MBO_ACTION, OP_INITIATOR_RESIDUAL,
    REG_CONTROL,
};
use crate::constants::timing::REQUEST_MS;
use crate::port::Port;
use crate::wait;

pub struct BtHba {
    pub port: Port,
    /// The grant: where the driver reaches it and where the adapter does,
    /// below 4 GiB.
    pub va: u64,
    pub dev: u64,
    /// The adapter's own SCSI ID, never a target.
    pub host_id: u8,
    /// The adapter did not come back from a reset; nothing more is sent.
    pub dead: bool,
}

impl BtHba {
    fn put<T>(&self, off: usize, v: T) {
        // SAFETY: `off` is a field of the mailboxes or the CCB, inside the
        // grant, which stays mapped while the driver lives.
        unsafe { write_volatile((self.va as usize + off) as *mut T, v) }
    }

    fn get<T: Copy>(&self, off: usize) -> T {
        // SAFETY: as above.
        unsafe { read_volatile((self.va as usize + off) as *const T) }
    }

    fn bus(&self, off: usize) -> u32 {
        (self.dev + off as u64) as u32
    }

    /// A request never completed: a hard reset drops it, and the adapter is
    /// given its mailboxes again (its own SCSI ID is its configuration's and
    /// survives the reset). One that does not take them is given up on.
    fn recover(&mut self) {
        if adapter::start(&self.port, self.va, self.dev).is_err() {
            self.dead = true;
        }
    }
}

impl Hba for BtHba {
    fn buffer(&mut self) -> &mut [u8] {
        // SAFETY: the data buffer is DATA_BYTES of the grant after its first
        // page, mapped for the driver's life; `&mut self` keeps it unique.
        unsafe {
            core::slice::from_raw_parts_mut((self.va as usize + DATA_OFF) as *mut u8, DATA_BYTES)
        }
    }

    fn exec(&mut self, at: Addr, cdb: &[u8], dir: Dir, len: usize) -> Result<Outcome, Fault> {
        if self.dead || cdb.is_empty() || cdb.len() > CDB_MAX || len > DATA_BYTES {
            return Err(Fault::Failed);
        }
        if at.target > MAX_TARGET || at.lun > MAX_LUN || at.target == self.host_id as u16 {
            return Err(Fault::NoTarget);
        }
        // SAFETY: the CCB, inside the grant as above.
        unsafe { write_bytes((self.va as usize + CCB_OFF) as *mut u8, 0, CCB_BYTES) };
        let direction = match dir {
            Dir::None => DIR_NONE,
            Dir::In => DIR_IN,
            Dir::Out => DIR_OUT,
        };
        self.put(CCB_OFF + CCB_OPCODE, OP_INITIATOR_RESIDUAL);
        self.put(CCB_OFF + CCB_DIRECTION, direction);
        self.put(CCB_OFF + CCB_CDB_LEN, cdb.len() as u8);
        self.put(CCB_OFF + CCB_SENSE_LEN, SENSE_BYTES);
        self.put(CCB_OFF + CCB_DATA_LEN, (len as u32).to_le());
        self.put(CCB_OFF + CCB_DATA, if len == 0 { 0 } else { self.bus(DATA_OFF).to_le() });
        self.put(CCB_OFF + CCB_TARGET, at.target as u8);
        self.put(CCB_OFF + CCB_LUN, at.lun as u8);
        for (i, &b) in cdb.iter().enumerate() {
            self.put(CCB_OFF + CCB_CDB + i, b);
        }
        self.put(CCB_OFF + CCB_SENSE, self.bus(SENSE_OFF).to_le());
        self.put(MBI_OFF + MBI_COMPLETION, COMP_FREE);
        self.put(MBO_OFF, self.bus(CCB_OFF).to_le());
        // The CCB and its address must be visible before the action that
        // hands them over.
        fence(Ordering::Release);
        self.put(MBO_OFF + MBO_ACTION, ACTION_START);
        if !adapter::kick(&self.port) {
            self.recover();
            return Err(Fault::Timeout);
        }
        if !wait::until(REQUEST_MS, || self.get::<u8>(MBI_OFF + MBI_COMPLETION) != COMP_FREE) {
            self.recover();
            return Err(Fault::Timeout);
        }
        // The CCB and the incoming mailbox are the driver's once the
        // completion code is seen.
        fence(Ordering::Acquire);
        let ccb = u32::from_le(self.get::<u32>(MBI_OFF));
        let host: u8 = self.get(MBI_OFF + MBI_HOST_STATUS);
        let target: u8 = self.get(MBI_OFF + MBI_TARGET_STATUS);
        let completion: u8 = self.get(MBI_OFF + MBI_COMPLETION);
        let residual = u32::from_le(self.get::<u32>(CCB_OFF + CCB_DATA_LEN));
        self.put(MBI_OFF + MBI_COMPLETION, COMP_FREE);
        self.port.write8(REG_CONTROL, CTL_INTERRUPT_RESET);
        if ccb != self.bus(CCB_OFF) {
            return Err(Fault::Failed);
        }
        match completion {
            COMP_DONE | COMP_ERROR => {}
            COMP_ABORTED | COMP_ABORTED_NOT_FOUND => return Err(Fault::Reset),
            _ => return Err(Fault::Failed),
        }
        match host {
            BT_SUCCESS | BT_LINKED_COMPLETE | BT_LINKED_COMPLETE_FLAG | BT_DATA_RUN => {}
            BT_SELECTION_TIMEOUT => return Err(Fault::NoTarget),
            BT_BUS_FREE | BT_HA_RESET | BT_OTHER_RESET => return Err(Fault::Reset),
            _ => return Err(Fault::Failed),
        }
        let mut o = Outcome::good();
        o.status = target;
        o.residual = residual.min(len as u32);
        if target == nonos_scsi::status::CHECK_CONDITION {
            o.sense_len = SENSE_MAX;
            for i in 0..SENSE_MAX {
                o.sense[i] = self.get(SENSE_OFF + i);
            }
        }
        Ok(o)
    }
}
