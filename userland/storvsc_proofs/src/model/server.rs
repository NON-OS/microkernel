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

//! The storage server on the channel's rings: each request the guest's
//! ring brings, checked against what Linux's storvsc_drv.c sends, and
//! answered with a completion on the host's ring. SCSI requests run on
//! nonos_scsi's simulated units, their data read from and written to the
//! pages the request names. Knobs make the host refuse, fall silent, or
//! send what a guest must pass over.

use std::collections::{HashMap, VecDeque};

use nonos_vmbus::model::HostRing;

use super::sim::{Unit, BUSY, CHECK, GOOD};
use super::{Grant, BASE_PFN, PAGE};

const COMPLETE_IO: u32 = 1;
const EXECUTE_SRB: u32 = 3;
const RESET_BUS: u32 = 6;
const BEGIN_INIT: u32 = 7;
const END_INIT: u32 = 8;
const QUERY_PROTOCOL_VERSION: u32 = 9;
const QUERY_PROPERTIES: u32 = 10;
const INBAND: u16 = 6;
const GPA_DIRECT: u16 = 9;
const COMP: u16 = 0xb;
const SRB_SUCCESS: u8 = 0x01;
const SRB_ERROR: u8 = 0x04;
const SRB_SELECTION_TIMEOUT: u8 = 0x0A;
const SRB_DATA_OVERRUN: u8 = 0x12;
const SRB_INVALID_LUN: u8 = 0x20;
const AUTOSENSE_VALID: u8 = 0x80;

/// One request as the host took it.
#[derive(Clone, Debug)]
pub struct Seen {
    pub ty: u16,
    pub txid: u64,
    pub op: u32,
    pub packet: Vec<u8>,
    /// A GPA-direct request's range: its length, its offset, its pages.
    pub range: Option<(u32, u32, Vec<u64>)>,
}

pub struct Server {
    /// By (target, LUN).
    pub units: HashMap<(u8, u8), Unit>,
    /// The protocol versions taken.
    pub versions: Vec<u16>,
    pub max_transfer: u32,
    /// Operations answered with a failing status.
    pub refuse: Vec<u32>,
    /// The next requests taken and never answered.
    pub silent: u32,
    /// Bus resets taken and never answered.
    pub silent_resets: bool,
    /// Before each answer, a completion for an id no one waits on.
    pub stale_first: bool,
    /// Before the next answer, a packet of the host's own, of this operation.
    pub unasked: Option<u32>,
    /// Before the next answer, a completion too long for any vstor packet.
    pub oversized: bool,
    /// The next SRB answered with this SRB status and SCSI status instead.
    pub srb_next: Option<(u8, u8)>,
    /// The next answer cut to 32 bytes.
    pub short_next: bool,
    /// The host takes nothing from the guest's ring.
    pub stalled: bool,
    pub seen: Vec<Seen>,
    pub resets: u32,
    out: VecDeque<(u16, u64, Vec<u8>)>,
}

impl Server {
    pub fn new(units: Vec<((u8, u8), Unit)>) -> Self {
        Self {
            units: units.into_iter().collect(),
            versions: vec![0x0602, 0x0600],
            max_transfer: 0x80000,
            refuse: Vec::new(),
            silent: 0,
            silent_resets: false,
            stale_first: false,
            unasked: None,
            oversized: false,
            srb_next: None,
            short_next: false,
            stalled: false,
            seen: Vec::new(),
            resets: 0,
            out: VecDeque::new(),
        }
    }

    /// Everything waiting on the guest's ring taken and answered.
    pub fn run(&mut self, g: Grant, pfns: &[u64], in_at: usize) {
        if self.stalled {
            return;
        }
        assert_eq!(pfns[0], BASE_PFN, "the rings are the grant's first pages");
        assert!(pfns.windows(2).all(|w| w[1] == w[0] + 1), "the GPADL's pages in order");
        let mut m = g.pages(pfns.len() * PAGE);
        let guest = HostRing { hdr: 0, len: ((in_at - 1) * PAGE) as u32 };
        let host = HostRing { hdr: in_at * PAGE, len: ((pfns.len() - in_at - 1) * PAGE) as u32 };
        loop {
            while let Some((ty, txid, data)) = self.out.front() {
                if host.send(&mut m, *ty, *txid, data).is_none() {
                    break;
                }
                self.out.pop_front();
            }
            let Some((ty, flags, txid, desc, data)) = guest.recv(&mut m) else { break };
            assert_eq!(flags & 1, 1, "every request asks for its completion");
            self.take(g, ty, txid, &desc, &data);
        }
    }

    fn answer(&mut self, txid: u64, p: Vec<u8>) {
        if self.stale_first {
            self.out.push_back((COMP, txid ^ 0x5555, complete(0)));
        }
        if let Some(op) = self.unasked.take() {
            let mut u = vec![0u8; 64];
            u[..4].copy_from_slice(&op.to_le_bytes());
            self.out.push_back((INBAND, 0, u));
        }
        if std::mem::take(&mut self.oversized) {
            // 136 bytes on the ring: past any answer, short of twice one.
            self.out.push_back((COMP, txid, vec![0u8; 120]));
        }
        let mut p = p;
        if std::mem::take(&mut self.short_next) {
            p.truncate(32);
        }
        self.out.push_back((COMP, txid, p));
    }

    fn take(&mut self, g: Grant, ty: u16, txid: u64, desc: &[u8], data: &[u8]) {
        assert!(data.len() >= 64, "a whole vstor packet");
        let p = &data[..64];
        let op = u32_at(p, 0);
        assert_eq!(u32_at(p, 4), 1, "REQUEST_COMPLETION_FLAG");
        let range = (ty == GPA_DIRECT).then(|| gpa_range(desc));
        self.seen.push(Seen { ty, txid, op, packet: p.to_vec(), range: range.clone() });
        if op == RESET_BUS {
            self.resets += 1;
            if !self.silent_resets {
                self.answer(txid, complete(0));
            }
            return;
        }
        if self.silent > 0 {
            self.silent -= 1;
            return;
        }
        let refused = self.refuse.contains(&op);
        let a = match op {
            BEGIN_INIT | END_INIT => complete(u32::from(refused)),
            QUERY_PROTOCOL_VERSION => {
                let v = u16::from_le_bytes([p[12], p[13]]);
                assert_eq!(&p[14..16], &[0, 0], "revision 0");
                complete(if self.versions.contains(&v) && !refused { 0 } else { 0xC000_0001 })
            }
            QUERY_PROPERTIES => {
                let mut a = complete(u32::from(refused));
                a[16..18].copy_from_slice(&1u16.to_le_bytes());
                a[24..28].copy_from_slice(&self.max_transfer.to_le_bytes());
                a
            }
            EXECUTE_SRB => self.srb(g, ty, p, range),
            k => panic!("an operation the driver never sends: {k}"),
        };
        self.answer(txid, a);
    }

    fn srb(&mut self, g: Grant, ty: u16, p: &[u8], range: Option<(u32, u32, Vec<u64>)>) -> Vec<u8> {
        assert_eq!(u16::from_le_bytes([p[12], p[13]]), 48, "sizeof(struct vmscsi_request)");
        assert_eq!(p[21], 20, "sense of STORVSC_SENSE_BUFFER_SIZE");
        assert_eq!((p[50], p[51]), (0xFF, 0x20), "untagged, simple");
        assert_eq!(u32_at(p, 56), 60, "60 seconds");
        let flags = u32_at(p, 52);
        assert_eq!(flags & 0x10A, 0x10A, "synch off, queue action enabled, no freeze");
        let len = u32_at(p, 24);
        let (target, lun, cdb_len, data_in) = (p[18], p[19], p[20] as usize, p[22]);
        assert!((1..=16).contains(&cdb_len));
        let cdb = p[28..28 + cdb_len].to_vec();
        match data_in {
            0 => assert_eq!(flags & 0xC0, 0x80, "a write says DATA_OUT"),
            1 => assert_eq!(flags & 0xC0, 0x40, "a read says DATA_IN"),
            _ => assert_eq!((data_in, flags & 0xC0, len), (2, 0, 0), "no data, no direction"),
        }
        let pages = match (ty, range) {
            (INBAND, None) => {
                assert_eq!(len, 0, "data travels by page, never in band");
                Vec::new()
            }
            (GPA_DIRECT, Some((rlen, offset, pfns))) => {
                assert_eq!((rlen, offset), (len, 0), "the range is the transfer");
                assert_eq!(pfns.len(), (len as usize).div_ceil(PAGE), "exactly its pages");
                pfns
            }
            other => panic!("a request of type {} with range {:?}", other.0, other.1),
        };
        let mut a = complete(0);
        a[12..64].copy_from_slice(&p[12..64]);
        let present = self.units.contains_key(&(target, lun));
        if !present {
            let any = self.units.keys().any(|(t, _)| *t == target);
            a[14] = if any { SRB_INVALID_LUN } else { SRB_SELECTION_TIMEOUT };
            a[15] = 0;
            a[24..28].copy_from_slice(&0u32.to_le_bytes());
            return a;
        }
        let out = if data_in == 0 { gather(g, &pages, len as usize) } else { Vec::new() };
        let unit = self.units.get_mut(&(target, lun)).unwrap();
        let ans = unit.execute(&cdb, &out);
        let moved = match data_in {
            1 => {
                let n = ans.data.len().min(len as usize);
                scatter(g, &pages, &ans.data[..n]);
                n as u32
            }
            0 if ans.status == GOOD => len,
            _ => 0,
        };
        let (srb, scsi) = match ans.status {
            CHECK => (SRB_ERROR | AUTOSENSE_VALID, CHECK),
            GOOD if moved < len => (SRB_DATA_OVERRUN, GOOD),
            BUSY => (SRB_SUCCESS, BUSY),
            s => (SRB_SUCCESS, s),
        };
        (a[14], a[15]) = self.srb_next.take().unwrap_or((srb, scsi));
        a[24..28].copy_from_slice(&moved.to_le_bytes());
        a[28..48].fill(0);
        let n = ans.sense.len().min(20);
        a[28..28 + n].copy_from_slice(&ans.sense[..n]);
        a[21] = n as u8;
        a
    }
}

fn u32_at(p: &[u8], at: usize) -> u32 {
    u32::from_le_bytes(p[at..at + 4].try_into().unwrap())
}

/// A completion of the start's kind: COMPLETE_IO and its status.
pub fn complete(status: u32) -> Vec<u8> {
    let mut a = vec![0u8; 64];
    a[..4].copy_from_slice(&COMPLETE_IO.to_le_bytes());
    a[8..12].copy_from_slice(&status.to_le_bytes());
    a
}

/// A GPA-direct descriptor of one range (QEMU's vmbus_pkt_gpa_direct).
fn gpa_range(desc: &[u8]) -> (u32, u32, Vec<u64>) {
    assert_eq!(u32_at(desc, 4), 1, "one range");
    let pfns = desc[16..].chunks(8).map(|c| u64::from_le_bytes(c.try_into().unwrap())).collect();
    (u32_at(desc, 8), u32_at(desc, 12), pfns)
}

fn gather(g: Grant, pfns: &[u64], len: usize) -> Vec<u8> {
    let mut v = vec![0u8; len];
    for (i, chunk) in v.chunks_mut(PAGE).enumerate() {
        // SAFETY: a page of the grant (Grant::at_pfn checks).
        unsafe { std::ptr::copy_nonoverlapping(g.at_pfn(pfns[i]), chunk.as_mut_ptr(), chunk.len()) };
    }
    v
}

fn scatter(g: Grant, pfns: &[u64], data: &[u8]) {
    for (i, chunk) in data.chunks(PAGE).enumerate() {
        // SAFETY: as above.
        unsafe { std::ptr::copy_nonoverlapping(chunk.as_ptr(), g.at_pfn(pfns[i]), chunk.len()) };
    }
}
