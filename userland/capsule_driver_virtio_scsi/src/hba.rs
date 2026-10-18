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

//! The adapter as `nonos_scsi` drives it: one command at a time down the
//! request queue, over any register access (`CommonCfg`): the mapped
//! regions in the capsule, a model in the proofs.

use nonos_scsi::{Addr, Dir, Fault, Hba, Outcome, SENSE_MAX};
use nonos_virtio::common::{CommonCfg, NotifyArea};

use crate::constants::layout::{DATA_OFF, REQ_OFF, RESP_OFF};
use crate::constants::scsi::{
    lun_field, CDB_SIZE, REQUESTQ, REQ_CDB, REQ_ID, REQ_LEN, REQ_LUN, RESP_LEN, RESP_RESID,
    RESP_RESPONSE, RESP_SENSE, RESP_SENSE_LEN, RESP_STATUS, SENSE_SIZE, S_ABORTED, S_BAD_TARGET,
    S_BUSY, S_OK, S_RESET, S_TRANSPORT_FAILURE,
};
use crate::constants::timing::COMMAND_MS;
use crate::queue::{Ring, Seg};
use crate::setup::handshake::{self, Limits};
use crate::wait;

pub struct VirtioHba<R: CommonCfg> {
    pub common: R,
    pub device: R,
    pub notify: R,
    pub notify_area: NotifyArea,
    pub doorbell: usize,
    pub ring: Ring,
    /// The grant: where the driver reaches it and where the device does.
    pub va: u64,
    pub dev: u64,
    /// The largest data phase: the buffer, or less when the device says so.
    pub max_bytes: usize,
    next_id: u64,
    /// The device did not come back from a reset; nothing more is sent.
    pub dead: bool,
}

impl<R: CommonCfg> VirtioHba<R> {
    pub fn new(
        common: R,
        device: R,
        notify: R,
        area: NotifyArea,
        l: Limits,
        va: u64,
        dev: u64,
    ) -> Self {
        Self {
            common,
            device,
            notify,
            notify_area: area,
            doorbell: l.doorbell,
            ring: Ring::new(va, l.size),
            va,
            dev,
            max_bytes: l.max_bytes,
            next_id: 0,
            dead: false,
        }
    }

    fn bytes(&mut self, off: usize, len: usize) -> &mut [u8] {
        // SAFETY: `off..off + len` lies in the grant (constants::layout),
        // which stays mapped while the driver lives; `&mut self` keeps the
        // slice unique.
        unsafe { core::slice::from_raw_parts_mut((self.va as usize + off) as *mut u8, len) }
    }

    /// A request the device never returned: reset it, which stops it
    /// touching the ring and the buffers, and bring it back up on a fresh
    /// ring. A device that does not come back is given up on.
    fn recover(&mut self) {
        match handshake::run(&self.common, &self.device, self.notify_area, self.dev) {
            Ok(l) => {
                self.ring = Ring::new(self.va, l.size);
                self.doorbell = l.doorbell;
            }
            Err(_) => self.dead = true,
        }
    }
}

impl<R: CommonCfg> Hba for VirtioHba<R> {
    fn buffer(&mut self) -> &mut [u8] {
        let n = self.max_bytes;
        self.bytes(DATA_OFF, n)
    }

    fn exec(&mut self, at: Addr, cdb: &[u8], dir: Dir, len: usize) -> Result<Outcome, Fault> {
        if self.dead || cdb.len() > CDB_SIZE || len > self.max_bytes {
            return Err(Fault::Failed);
        }
        self.next_id = self.next_id.wrapping_add(1);
        let id = self.next_id;
        let req = self.bytes(REQ_OFF, REQ_LEN);
        req.fill(0);
        req[REQ_LUN..REQ_LUN + 8].copy_from_slice(&lun_field(at.target, at.lun));
        req[REQ_ID..REQ_ID + 8].copy_from_slice(&id.to_le_bytes());
        req[REQ_CDB..REQ_CDB + cdb.len()].copy_from_slice(cdb);
        self.bytes(RESP_OFF, RESP_LEN).fill(0);
        let seg = |off: usize, len: usize, write: bool| Seg {
            addr: self.dev + off as u64,
            len: len as u32,
            write,
        };
        let mut chain = [seg(REQ_OFF, REQ_LEN, false); 3];
        let mut n = 1;
        if dir == Dir::Out && len > 0 {
            chain[n] = seg(DATA_OFF, len, false);
            n += 1;
        }
        chain[n] = seg(RESP_OFF, RESP_LEN, true);
        n += 1;
        if dir == Dir::In && len > 0 {
            chain[n] = seg(DATA_OFF, len, true);
            n += 1;
        }
        self.ring.post(&chain[..n]);
        self.notify.w16(self.doorbell, REQUESTQ);
        if !wait::until(COMMAND_MS, || self.ring.returned()) {
            self.recover();
            return Err(Fault::Timeout);
        }
        let _ = self.ring.take();
        let resp = self.bytes(RESP_OFF, RESP_LEN);
        match resp[RESP_RESPONSE] {
            S_OK => {}
            S_BAD_TARGET => return Err(Fault::NoTarget),
            S_RESET | S_ABORTED | S_TRANSPORT_FAILURE => return Err(Fault::Reset),
            S_BUSY => return Err(Fault::Busy),
            _ => return Err(Fault::Failed),
        }
        let le32 =
            |at: usize| u32::from_le_bytes([resp[at], resp[at + 1], resp[at + 2], resp[at + 3]]);
        let mut o = Outcome::good();
        o.status = resp[RESP_STATUS];
        o.residual = le32(RESP_RESID);
        o.sense_len = (le32(RESP_SENSE_LEN) as usize).min(SENSE_SIZE).min(SENSE_MAX);
        o.sense[..o.sense_len].copy_from_slice(&resp[RESP_SENSE..RESP_SENSE + o.sense_len]);
        Ok(o)
    }
}
