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

//! The request queue: each new chain read from the descriptor table the
//! driver placed, the request answered, the response and data written
//! back, the chain returned on the used ring.

use std::ptr::{read_volatile, write_volatile};

use super::sim::{check, Answer};
use super::Scsi;
use crate::common::STATUS_DRIVER_OK;

/// One buffer as the device saw it: address, length, written by the device.
pub type Seg = (u64, u32, bool);

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Chain {
    pub segs: Vec<Seg>,
    pub lun: [u8; 8],
    pub id: u64,
    pub cdb: Vec<u8>,
}

const REQUESTQ: usize = 2;
/// Each doorbell is the queue's notify offset times the multiplier the
/// proofs give the driver.
pub const MULTIPLIER: u32 = 4;

fn rd<T: Copy>(addr: u64) -> T {
    // SAFETY: every address the driver hands the device is in the grant the
    // test allocated, whose bus address is its host address.
    unsafe { read_volatile(addr as *const T) }
}

fn wr<T: Copy>(addr: u64, v: T) {
    // SAFETY: as above.
    unsafe { write_volatile(addr as *mut T, v) }
}

impl Scsi {
    pub fn notify(&self, off: usize, value: u16) {
        assert_eq!(off, REQUESTQ * MULTIPLIER as usize, "the request queue's doorbell");
        assert_eq!(value, REQUESTQ as u16, "the doorbell names its queue");
        let st = self.common.state();
        assert!(st.status & STATUS_DRIVER_OK != 0, "a doorbell before DRIVER_OK");
        let q = st.queues[REQUESTQ];
        drop(st);
        assert_eq!(q.enable, 1, "a doorbell on a disabled queue");
        self.rest.borrow_mut().notifies += 1;
        loop {
            let avail: u16 = rd(q.driver + 2);
            let mut rest = self.rest.borrow_mut();
            if rest.seen_avail == avail || rest.stall {
                return;
            }
            let slot = rest.seen_avail % q.size;
            rest.seen_avail = rest.seen_avail.wrapping_add(1);
            drop(rest);
            let head: u16 = rd(q.driver + 4 + slot as u64 * 2);
            let written = self.serve(q.desc, head, q.size);
            let mut rest = self.rest.borrow_mut();
            let u = rest.used_idx % q.size;
            wr(q.device + 4 + u as u64 * 8, head as u32);
            wr(q.device + 8 + u as u64 * 8, written);
            rest.used_idx = rest.used_idx.wrapping_add(1);
            std::sync::atomic::fence(std::sync::atomic::Ordering::Release);
            wr(q.device + 2, rest.used_idx);
        }
    }

    /// Walk the chain at `head`, answer it, and return the bytes written.
    fn serve(&self, desc: u64, head: u16, size: u16) -> u32 {
        let mut segs = Vec::new();
        let mut i = head;
        loop {
            assert!(i < size, "a descriptor past the ring");
            let d = desc + i as u64 * 16;
            let (addr, len, flags, next): (u64, u32, u16, u16) =
                (rd(d), rd(d + 8), rd(d + 12), rd(d + 14));
            segs.push((addr, len, flags & 2 != 0));
            if flags & 1 == 0 {
                break;
            }
            i = next;
            assert!(segs.len() <= 4, "a chain that does not end");
        }
        let first_write = segs.iter().position(|s| s.2).expect("a chain with a response");
        assert!(segs[first_write..].iter().all(|s| s.2), "the device-written buffers come last");
        let (req, len, _) = segs[0];
        assert_eq!(len, 51, "the request header with a 32-byte CDB");
        let read = |a: u64, n: usize| (0..n).map(|k| rd::<u8>(a + k as u64)).collect::<Vec<u8>>();
        let header = read(req, 51);
        let lun: [u8; 8] = header[0..8].try_into().unwrap();
        let id = u64::from_le_bytes(header[8..16].try_into().unwrap());
        let cdb = header[19..51].to_vec();
        let out: Vec<u8> =
            segs[1..first_write].iter().flat_map(|&(a, n, _)| read(a, n as usize)).collect();
        let (resp, resp_len, _) = segs[first_write];
        assert_eq!(resp_len, 108, "the response with 96 bytes of sense");
        let ins = &segs[first_write + 1..];
        self.rest.borrow_mut().chains.push(Chain { segs: segs.clone(), lun, id, cdb: cdb.clone() });
        let mut response = 0u8;
        let mut answer = Answer { status: 0, sense: vec![], data: vec![] };
        let forced = self.rest.borrow_mut().response.take();
        if let Some(r) = forced {
            response = r;
        } else {
            assert_eq!(lun[0], 1, "the LUN field's first byte");
            assert_eq!(lun[2] & 0xC0, 0x40, "flat space addressing");
            let (t, l) = (lun[1] as u16, u16::from(lun[2] & 0x3F) << 8 | lun[3] as u16);
            let mut rest = self.rest.borrow_mut();
            if !rest.units.keys().any(|k| k.0 == t) {
                response = 3;
            } else if let Some(u) = rest.units.get_mut(&(t, l)) {
                answer = u.execute(&cdb, &out);
            } else if cdb[0] == 0x12 {
                let mut d = vec![0u8; 36];
                d[0] = 0x7F;
                answer = Answer { status: 0, sense: vec![], data: d };
            } else {
                answer = check(0x5, 0x25, 0x00);
            }
        }
        let in_len: usize = ins.iter().map(|s| s.1 as usize).sum();
        let mut at = 0usize;
        for &(a, n, _) in ins {
            for k in 0..n as usize {
                if at + k < answer.data.len() {
                    wr(a + k as u64, answer.data[at + k]);
                }
            }
            at += n as usize;
        }
        let moved = answer.data.len().min(in_len);
        let resid = (in_len - moved) as u32;
        wr(resp, answer.sense.len() as u32);
        wr(resp + 4, resid);
        wr(resp + 10, answer.status);
        wr(resp + 11, response);
        for (k, &b) in answer.sense.iter().enumerate() {
            wr(resp + 12 + k as u64, b);
        }
        108 + moved as u32
    }
}
