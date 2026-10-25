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

//! The channel as `nonos_scsi` drives it: one request at a time down the
//! guest's ring, its completion found on the host's ring by its id. The
//! host's own packets met on the way (a bus changed, a device gone) are
//! noted and passed over; a completion for an id no longer waited on, one
//! the host sent after its request was given up on, is dropped.

use nonos_scsi::{Addr, Dir, Fault, Hba, Outcome};
use nonos_vmbus::packet::{
    gpa_direct, PACKET_COMP, PACKET_DATA_INBAND, PACKET_DATA_USING_GPA_DIRECT,
};
use nonos_vmbus::{Channel, Header, Host, RingError, Shared};

use crate::constants::layout::{DATA_BYTES, DATA_PAGES, PAGE};
use crate::constants::timing::{COMMAND_MS, INIT_MS};
use crate::constants::vstor::{
    OP_ENUMERATE_BUS, OP_REMOVE_DEVICE, OP_RESET_BUS, PACKET, RESET_ID,
};
use crate::vstor::{self, Packet};

/// The largest packet the host sends this driver: a header and a vstor
/// packet; anything longer is skipped unread.
pub const IN_MAX: usize = 16 + 8 + PACKET;

pub struct VmHba<H: Host, M: Shared> {
    pub host: H,
    /// The two rings' pages.
    pub rings: M,
    pub chan: Channel,
    /// The data buffer: where the driver reaches it and the first of its
    /// page numbers, as the host reaches it.
    data: *mut u8,
    data_pfn: u64,
    /// The largest transfer the host takes (QUERY_PROPERTIES), at most
    /// DATA_BYTES.
    pub limit: usize,
    next_id: u64,
    /// The host said its bus changed: the units may differ now.
    pub changed: bool,
    /// The host stopped answering, even a bus reset: nothing more is sent.
    pub dead: bool,
}

/// What waiting for one id came to.
enum Wait {
    Got(usize),
    Silent,
    Broken,
}

impl<H: Host, M: Shared> VmHba<H, M> {
    /// # Safety
    /// `data` points to DATA_BYTES the host reaches at page `data_pfn` on,
    /// mapped for the value's life and touched by no one else but the host.
    pub unsafe fn new(host: H, rings: M, chan: Channel, data: *mut u8, data_pfn: u64) -> Self {
        Self { host, rings, chan, data, data_pfn, limit: DATA_BYTES, next_id: 0, changed: false, dead: false }
    }

    /// Send `body` as one packet of `id`: in band, or with the first `len`
    /// bytes of the data buffer named page by page. A ring with no room is
    /// waited on until `until`.
    fn send(&mut self, id: u64, body: &[u8], len: usize, until: u64) -> Result<(), Fault> {
        let mut desc = [0u8; 16 + 8 * DATA_PAGES];
        let (ty, desc) = if len == 0 {
            (PACKET_DATA_INBAND, &desc[..0])
        } else {
            let mut pfns = [0u64; DATA_PAGES];
            let pages = len.div_ceil(PAGE);
            for (i, p) in pfns[..pages].iter_mut().enumerate() {
                *p = self.data_pfn + i as u64;
            }
            let n = gpa_direct(&mut desc, len as u32, 0, &pfns[..pages]).ok_or(Fault::Failed)?;
            (PACKET_DATA_USING_GPA_DIRECT, &desc[..n])
        };
        let header = Header::new(ty, nonos_vmbus::packet::FLAG_REQUEST_COMPLETION, id);
        loop {
            match self.chan.send(&mut self.host, &mut self.rings, &header, desc, &[body]) {
                Ok(()) => return Ok(()),
                Err(RingError::Full) if self.host.now_ms() < until => self.host.idle(),
                Err(RingError::Full) => return Err(Fault::Timeout),
                Err(_) => return Err(Fault::Failed),
            }
        }
    }

    /// The completion of `id` into `out`, its vstor packet's length.
    fn wait(&mut self, id: u64, out: &mut [u8; IN_MAX], until: u64) -> Wait {
        loop {
            match self.chan.recv(&mut self.host, &mut self.rings, out) {
                Ok(Some((h, n))) => {
                    let start = h.data_at.min(n);
                    if h.ty == PACKET_COMP && h.transaction_id == id {
                        out.copy_within(start..n, 0);
                        return Wait::Got(n - start);
                    }
                    if h.transaction_id == 0 {
                        self.unasked(&out[start..n]);
                    }
                }
                Ok(None) if self.host.now_ms() < until => self.host.idle(),
                Ok(None) => return Wait::Silent,
                Err(RingError::TooBig) => {
                    if self.chan.inb.skip(&mut self.rings).is_err() {
                        return Wait::Broken;
                    }
                }
                Err(_) => return Wait::Broken,
            }
        }
    }

    /// A packet the host sent of its own accord.
    fn unasked(&mut self, body: &[u8]) {
        if let Some(r) = vstor::reply(body) {
            if r.operation == OP_ENUMERATE_BUS || r.operation == OP_REMOVE_DEVICE {
                self.changed = true;
            }
        }
    }

    /// One request and its completion, the completion's vstor packet into
    /// `out`.
    pub fn transact(
        &mut self,
        id: u64,
        body: &Packet,
        len: usize,
        ms: u64,
        out: &mut [u8; IN_MAX],
    ) -> Result<usize, Fault> {
        let until = self.host.now_ms().saturating_add(ms);
        self.send(id, body, len, until)?;
        match self.wait(id, out, until) {
            Wait::Got(n) => Ok(n),
            Wait::Silent => Err(Fault::Timeout),
            Wait::Broken => {
                self.dead = true;
                Err(Fault::Failed)
            }
        }
    }

    /// A request never completed: reset the bus, which the host answers
    /// once every request on it is done or dropped, so the buffer is the
    /// driver's again. A host that does not answer that is given up on.
    fn recover(&mut self) {
        let mut out = [0u8; IN_MAX];
        let reset = vstor::op(OP_RESET_BUS);
        match self.transact(RESET_ID, &reset, 0, INIT_MS, &mut out) {
            Ok(_) => {}
            Err(_) => self.dead = true,
        }
    }

    fn take_id(&mut self) -> u64 {
        self.next_id = next_id(self.next_id);
        self.next_id
    }
}

/// The id after `id`: 1 upward, wrapping short of the start's and the
/// reset's ids; 0 is the host's own.
pub fn next_id(id: u64) -> u64 {
    if id >= RESET_ID - 1 {
        1
    } else {
        id + 1
    }
}

impl<H: Host, M: Shared> Hba for VmHba<H, M> {
    fn buffer(&mut self) -> &mut [u8] {
        // SAFETY: DATA_BYTES at `data`, as `new` was promised; `&mut self`
        // keeps the slice unique, and the host writes it only while a
        // request is out, inside `exec`.
        unsafe { core::slice::from_raw_parts_mut(self.data, self.limit) }
    }

    fn exec(&mut self, at: Addr, cdb: &[u8], dir: Dir, len: usize) -> Result<Outcome, Fault> {
        if self.dead || len > self.limit {
            return Err(Fault::Failed);
        }
        let body = vstor::srb(at, cdb, dir, len as u32).ok_or(Fault::Failed)?;
        let id = self.take_id();
        let mut out = [0u8; IN_MAX];
        match self.transact(id, &body, len, COMMAND_MS, &mut out) {
            Ok(n) => vstor::outcome(&out[..n], cdb, len as u32),
            Err(Fault::Timeout) => {
                self.recover();
                Err(Fault::Timeout)
            }
            Err(e) => Err(e),
        }
    }
}
