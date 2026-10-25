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

//! The storage server on the channel's rings, written from the host side
//! Microsoft publishes: OpenVMM's storvsp (vm/devices/storage/storvsp/src/
//! lib.rs, cited "storvsp:LINE") and its protocol crate (storvsp_protocol,
//! "proto:LINE"), with SCSI answers in scsidisk's shapes ("scsidisk:LINE").
//! What Linux's storvsc_drv.c sends is checked apart, marked "Linux sends".
//! SCSI requests to a present LUN run on nonos_scsi's simulated units, their
//! data read from and written to the pages the request names. Knobs make
//! the host refuse, fall silent, or send what a guest must pass over.

use std::collections::{HashMap, VecDeque};

use nonos_vmbus::model::HostRing;

use super::sim::{Unit, CHECK, GOOD};
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
const SRB_INVALID_REQUEST: u8 = 0x06;
const SRB_INVALID_LUN: u8 = 0x20;
const AUTOSENSE_VALID: u8 = 0x80;
/// proto:46-49: WIN6, WIN7, WIN8 and BLUE (WIN8.1); storvsp:235-251 takes
/// these alone, so THRESHOLD (WIN10, 6.2) is refused.
pub const STORVSP_VERSIONS: [u16; 4] = [0x0200, 0x0402, 0x0501, 0x0600];
/// proto:64.
const REVISION_MISMATCH: u32 = 0xC000_0059;
/// proto:144: the largest request; a completion is the header and as many
/// request bytes as the guest sent, up to this (storvsp:422, 511).
const REQUEST_LEN_MAX: usize = 0x34;

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
            versions: STORVSP_VERSIONS.to_vec(),
            // storvsp:1024.
            max_transfer: 0x40000,
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
            self.out.push_back((COMP, txid ^ 0x5555, complete(0, REQUEST_LEN_MAX)));
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
        // Linux sends: every request is a whole vstor_packet asking for its
        // completion.
        assert!(data.len() >= 64, "a whole vstor packet");
        let p = &data[..64];
        let op = u32_at(p, 0);
        assert_eq!(u32_at(p, 4), 1, "REQUEST_COMPLETION_FLAG");
        let range = (ty == GPA_DIRECT).then(|| gpa_range(desc));
        self.seen.push(Seen { ty, txid, op, packet: p.to_vec(), range: range.clone() });
        // storvsp:422: the request is what follows the 12-byte header, held
        // to REQUEST_LEN_MAX; every answer is that long again (storvsp:511).
        let request = (data.len() - 12).min(REQUEST_LEN_MAX);
        if op == RESET_BUS {
            self.resets += 1;
            // storvsp:1349-1351: a reset is answered SUCCESS.
            if !self.silent_resets {
                self.answer(txid, complete(0, request));
            }
            return;
        }
        if self.silent > 0 {
            self.silent -= 1;
            return;
        }
        let refused = self.refuse.contains(&op);
        let a = match op {
            // storvsp:955-965, 1022: SUCCESS, or INVALID_DEVICE_STATE out of
            // order (a refusal here).
            BEGIN_INIT | END_INIT => complete(if refused { INVALID_DEVICE_STATE } else { 0 }, request),
            QUERY_PROTOCOL_VERSION => {
                let v = u16::from_le_bytes([p[12], p[13]]);
                assert_eq!(&p[14..16], &[0, 0], "Linux sends revision 0");
                // storvsp:977-995: a version it knows, else REVISION_MISMATCH.
                let ok = self.versions.contains(&v) && !refused;
                complete(if ok { 0 } else { REVISION_MISMATCH }, request)
            }
            QUERY_PROPERTIES => {
                // storvsp:1020-1030: max_channel_cnt, then max_transfer_bytes
                // at body offset 12 (packet byte 24).
                let mut a = complete(if refused { INVALID_DEVICE_STATE } else { 0 }, request);
                a[16..18].copy_from_slice(&1u16.to_le_bytes());
                a[24..28].copy_from_slice(&self.max_transfer.to_le_bytes());
                a
            }
            EXECUTE_SRB => self.srb(g, ty, p, range, request),
            k => panic!("an operation the driver never sends: {k}"),
        };
        self.answer(txid, a);
    }

    fn srb(
        &mut self,
        g: Grant,
        ty: u16,
        p: &[u8],
        range: Option<(u32, u32, Vec<u64>)>,
        request: usize,
    ) -> Vec<u8> {
        // Linux sends (storvsc_queuecommand): the request's own size, sense
        // of 20 bytes, untagged simple queueing, synchronous transfer off,
        // 60 seconds, the direction in data_in and srb_flags alike.
        assert_eq!(u16::from_le_bytes([p[12], p[13]]), 52, "sizeof(struct vmscsi_request), proto:143");
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
        // storvsp:366-377: one range at least the transfer long, or none and
        // no transfer; anything else fails the channel.
        let pages = match (ty, range) {
            (INBAND, None) => {
                assert_eq!(len, 0, "storvsp:372: a transfer with no range");
                Vec::new()
            }
            (GPA_DIRECT, Some((rlen, offset, pfns))) => {
                assert!(rlen >= len, "storvsp:372: a range shorter than the transfer");
                assert_eq!(offset, 0, "the buffer starts its first page");
                assert_eq!(pfns.len(), (rlen as usize).div_ceil(PAGE), "exactly its pages");
                pfns
            }
            other => panic!("a request of type {} with range {:?}", other.0, other.1),
        };
        let r = if cdb[0] == REPORT_LUNS {
            self.report_luns(g, &pages, target, len)
        } else if self.units.contains_key(&(target, lun)) {
            self.present(g, &pages, target, lun, &cdb, data_in, len)
        } else if cdb[0] == INQUIRY {
            absent_inquiry(g, &pages, &cdb, data_in, len, lun)
        } else {
            // storvsp:741-745: any other command to an absent LUN.
            Result { srb: SRB_INVALID_LUN, scsi: CHECK, tx: 0, sense: None }
        };
        let (srb, scsi) = self.srb_next.take().unwrap_or((r.srb, r.scsi));
        respond(Result { srb, scsi, ..r }, request)
    }

    /// storvsp:610-661: the controller answers REPORT LUNS itself, from the
    /// LUNs it has on that target, whether LUN 0 is one or not.
    fn report_luns(&self, g: Grant, pages: &[u64], target: u8, len: u32) -> Result {
        let mut luns: Vec<u8> =
            self.units.keys().filter(|(t, _)| *t == target).map(|(_, l)| *l).collect();
        luns.sort_unstable();
        let mut data = vec![0u8; 8 * (luns.len() + 1)];
        data[..4].copy_from_slice(&(luns.len() as u32 * 8).to_be_bytes());
        for (i, l) in luns.iter().enumerate() {
            data[8 * (i + 1)..8 * (i + 1) + 2].copy_from_slice(&u16::from(*l).to_be_bytes());
        }
        if (len as usize) < 8 {
            return Result { srb: SRB_SUCCESS, scsi: GOOD, tx: 0, sense: None };
        }
        let tx = (len as usize).min(data.len());
        scatter(g, pages, &data[..tx]);
        Result { srb: SRB_SUCCESS, scsi: GOOD, tx: tx as u32, sense: None }
    }

    /// A command to a present LUN, answered by its simulated unit in the
    /// shapes scsidisk gives (scsidisk:994-1107): success with what moved;
    /// a CHECK CONDITION with its sense and nothing moved, INVALID_REQUEST
    /// for an illegal request and ERROR otherwise; another status (BUSY, as
    /// RESERVATION CONFLICT at scsidisk:1100) as ERROR with no sense.
    #[allow(clippy::too_many_arguments)]
    fn present(
        &mut self,
        g: Grant,
        pages: &[u64],
        target: u8,
        lun: u8,
        cdb: &[u8],
        data_in: u8,
        len: u32,
    ) -> Result {
        let out = if data_in == 0 { gather(g, pages, len as usize) } else { Vec::new() };
        let ans = self.units.get_mut(&(target, lun)).unwrap().execute(cdb, &out);
        match ans.status {
            GOOD => {
                let tx = if data_in == 1 {
                    let n = ans.data.len().min(len as usize);
                    scatter(g, pages, &ans.data[..n]);
                    n as u32
                } else {
                    len
                };
                Result { srb: SRB_SUCCESS, scsi: GOOD, tx, sense: None }
            }
            CHECK => {
                let illegal = ans.sense.get(2).is_some_and(|k| k & 0x0F == 0x5);
                let srb = if illegal { SRB_INVALID_REQUEST } else { SRB_ERROR };
                Result { srb, scsi: CHECK, tx: 0, sense: Some(ans.sense) }
            }
            s => Result { srb: SRB_ERROR, scsi: s, tx: 0, sense: None },
        }
    }
}

/// What the host made of a SCSI request.
struct Result {
    srb: u8,
    scsi: u8,
    tx: u32,
    sense: Option<Vec<u8>>,
}

const REPORT_LUNS: u8 = 0xA0;
const INQUIRY: u8 = 0x12;
/// proto:67.
const INVALID_DEVICE_STATE: u32 = 0xC000_0184;

/// Fixed-format sense: ILLEGAL REQUEST, INVALID FIELD IN CDB (24h/00h).
fn illegal_request() -> Vec<u8> {
    let mut s = vec![0u8; 18];
    (s[0], s[2], s[7], s[12]) = (0x70, 0x05, 10, 0x24);
    s
}

/// storvsp:666-737: INQUIRY to an absent LUN. A VPD page, a buffer short of
/// the allocation length, a read that is not one, or an allocation length
/// short of the header is an illegal request; otherwise standard data
/// saying no device is there (peripheral 0x7F), names only at LUN 0.
fn absent_inquiry(g: Grant, pages: &[u64], cdb: &[u8], data_in: u8, len: u32, lun: u8) -> Result {
    let alloc = u16::from_be_bytes([cdb[3], cdb[4]]) as usize;
    let vpd = cdb[1] & 1 != 0 || cdb[2] != 0;
    if (len as usize) < alloc || data_in != 1 || alloc < 5 || vpd {
        return Result { srb: SRB_INVALID_REQUEST, scsi: CHECK, tx: 0, sense: Some(illegal_request()) };
    }
    let mut data = [0u8; 36];
    (data[0], data[2], data[3], data[4]) = (0x7F, 0x06, 0x02, 31);
    if lun == 0 {
        data[8..16].copy_from_slice(b"Msft    ");
        data[16..32].copy_from_slice(b"Virtual Disk    ");
    }
    let tx = alloc.min(data.len());
    scatter(g, pages, &data[..tx]);
    Result { srb: SRB_SUCCESS, scsi: GOOD, tx: tx as u32, sense: None }
}

/// storvsp:1307-1330: the completion is a fresh request, not the guest's
/// echoed: its length, the statuses (autosense set when there is sense),
/// what moved, a CDB length of 16, a sense length of 20 whatever the sense,
/// and the sense; the packet's status from the SRB status (storvsp:1135).
fn respond(r: Result, request: usize) -> Vec<u8> {
    let mut a = complete(nt_status(r.srb), request);
    a[12..14].copy_from_slice(&52u16.to_le_bytes());
    a[14] = r.srb | if r.sense.is_some() { AUTOSENSE_VALID } else { 0 };
    a[15] = r.scsi;
    (a[20], a[21]) = (16, 20);
    a[24..28].copy_from_slice(&r.tx.to_le_bytes());
    if let Some(sense) = r.sense {
        let n = sense.len().min(20);
        a[28..28 + n].copy_from_slice(&sense[..n]);
    }
    a
}

/// storvsp:1135-1151.
fn nt_status(srb: u8) -> u32 {
    match srb & 0x3F {
        SRB_SUCCESS => 0,
        0x12 => 0x8000_0005,
        0x0A => 0xC000_009D,
        0x02 => 0xC000_0120,
        0x09 => 0xC000_00B5,
        SRB_INVALID_REQUEST => 0xC000_0010,
        SRB_INVALID_LUN => 0xC000_00C0,
        _ => 0xC000_0185,
    }
}

fn u32_at(p: &[u8], at: usize) -> u32 {
    u32::from_le_bytes(p[at..at + 4].try_into().unwrap())
}

/// A completion: COMPLETE_IO, its status, then `request` bytes (storvsp:
/// 511-525: zero padded to the request's own size).
pub fn complete(status: u32, request: usize) -> Vec<u8> {
    let mut a = vec![0u8; 12 + request];
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
