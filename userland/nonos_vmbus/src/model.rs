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

//! A model of a VMBus host's side of one channel, written from QEMU's
//! hw/hyperv/vmbus.c (rb_idx_delta, vmbus_channel_reserve,
//! ringbuf_send_update_idx, vmbus_channel_recv_start and _pop) and nothing
//! of this crate's ring code: the host reads the guest's ring and writes
//! its own as QEMU does, so the two sides are checked against each other.

use std::collections::VecDeque;
use std::vec::Vec;

use crate::channel::{Host, PostError};
use crate::mem::Shared;
use crate::msg::MESSAGE_MAX;

const PAGE: usize = 4096;

/// QEMU's rb_idx_delta.
fn delta(idx1: u32, idx2: u32, len: u32, allow_catchup: bool) -> u32 {
    let d = idx2 + len - idx1 - u32::from(!allow_catchup);
    if d >= len {
        d - len
    } else {
        d
    }
}

/// One ring as the host sees it.
pub struct HostRing {
    pub hdr: usize,
    pub len: u32,
}

impl HostRing {
    fn idx(&self, m: &impl Shared, f: usize) -> u32 {
        m.load32(self.hdr + f)
    }

    fn io(&self, m: &mut impl Shared, at: u32, buf: &mut [u8], write: bool) {
        for (i, b) in buf.iter_mut().enumerate() {
            let off = self.hdr + PAGE + ((at as usize + i) % self.len as usize);
            if write {
                m.write(off, core::slice::from_ref(b));
            } else {
                m.read(off, core::slice::from_mut(b));
            }
        }
    }

    /// The host as producer (vmbus_channel_reserve + send): a packet of
    /// `ty` with `data`, or false for no room (pending_send_sz then set).
    /// Whether it signalled the guest.
    pub fn send(&self, m: &mut impl Shared, ty: u16, txid: u64, data: &[u8]) -> Option<bool> {
        self.send_packet(m, ty, 0, txid, &[], data)
    }

    /// The same, with `flags` and a descriptor `desc` (whole qwords) between
    /// the header and the data: a transfer-pages packet's ranges, as
    /// OpenVMM's vmbus_ring writes them (TransferPageHeader, then ranges).
    pub fn send_packet(
        &self,
        m: &mut impl Shared,
        ty: u16,
        flags: u16,
        txid: u64,
        desc: &[u8],
        data: &[u8],
    ) -> Option<bool> {
        assert!(desc.len().is_multiple_of(8), "a descriptor of whole qwords");
        let offset_qwords = 2 + desc.len() / 8;
        let len_qwords = offset_qwords + data.len().div_ceil(8);
        let total = (len_qwords * 8 + 8) as u32;
        let wr = self.idx(m, 0);
        let rd = self.idx(m, 4);
        if total > delta(wr, rd, self.len, false) {
            m.store32(self.hdr + 12, total);
            return None;
        }
        let mut pkt = vec![0u8; len_qwords * 8 + 8];
        pkt[0..2].copy_from_slice(&ty.to_le_bytes());
        pkt[2..4].copy_from_slice(&(offset_qwords as u16).to_le_bytes());
        pkt[4..6].copy_from_slice(&(len_qwords as u16).to_le_bytes());
        pkt[6..8].copy_from_slice(&flags.to_le_bytes());
        pkt[8..16].copy_from_slice(&txid.to_le_bytes());
        pkt[16..16 + desc.len()].copy_from_slice(desc);
        let at = offset_qwords * 8;
        pkt[at..at + data.len()].copy_from_slice(data);
        let tail = len_qwords * 8;
        pkt[tail..tail + 8].copy_from_slice(&(u64::from(wr) << 32).to_le_bytes());
        self.io(m, wr, &mut pkt, true);
        let new_wr = (wr + total) % self.len;
        m.store32(self.hdr, new_wr);
        m.store32(self.hdr + 12, 0);
        // Notify unless masked or the reader has not caught up.
        let masked = self.idx(m, 8) != 0;
        let behind = delta(self.idx(m, 4), new_wr, self.len, true) > total;
        Some(!masked && !behind)
    }

    /// The host as consumer (recv_start + recv_pop): the next packet's
    /// type, flags, transaction id, descriptor and data, or None.
    #[allow(clippy::type_complexity)]
    pub fn recv(&self, m: &mut impl Shared) -> Option<(u16, u16, u64, Vec<u8>, Vec<u8>)> {
        let rd = self.idx(m, 4);
        let wr = self.idx(m, 0);
        assert!(
            rd < self.len && wr < self.len,
            "guest index out of the ring"
        );
        let avail = delta(rd, wr, self.len, true);
        if avail == 0 {
            return None;
        }
        let mut hdr = [0u8; 16];
        self.io(m, rd, &mut hdr, false);
        let ty = u16::from_le_bytes([hdr[0], hdr[1]]);
        let off = u16::from_le_bytes([hdr[2], hdr[3]]) as usize * 8;
        let len = u16::from_le_bytes([hdr[4], hdr[5]]) as usize * 8;
        let flags = u16::from_le_bytes([hdr[6], hdr[7]]);
        let txid = u64::from_le_bytes(hdr[8..16].try_into().unwrap());
        assert!(
            len + 8 <= avail as usize && off >= 16 && off <= len,
            "a malformed guest packet"
        );
        let mut pkt = vec![0u8; len + 8];
        self.io(m, rd, &mut pkt, false);
        let trailer = u64::from_le_bytes(pkt[len..len + 8].try_into().unwrap());
        assert_eq!(
            trailer >> 32,
            u64::from(rd),
            "the trailer names where the packet began"
        );
        m.store32(self.hdr + 4, (rd + (len + 8) as u32) % self.len);
        Some((
            ty,
            flags,
            txid,
            pkt[16..off].to_vec(),
            pkt[off..len].to_vec(),
        ))
    }
}

/// What the host answers each step with.
#[derive(Clone, Copy)]
pub struct Script {
    pub gpadl_status: Option<u32>,
    pub open_status: Option<u32>,
    pub torndown: bool,
    /// Posts refused as busy before each is taken.
    pub busy: u32,
}

impl Default for Script {
    fn default() -> Self {
        Self {
            gpadl_status: Some(0),
            open_status: Some(0),
            torndown: true,
            busy: 0,
        }
    }
}

/// The host behind the kernel: the channel messages the driver posts, the
/// answers it gives, and the clock.
pub struct Model {
    pub script: Script,
    pub posted: Vec<Vec<u8>>,
    pub answers: VecDeque<Vec<u8>>,
    pub signals: u32,
    pub now: u64,
    busy_left: u32,
    gpadl_pfns: Vec<u64>,
    gpadl_want: usize,
    pub gpadls: Vec<(u32, Vec<u64>)>,
    pub open: Option<(u32, u32, u32)>,
    pub refuse: bool,
    /// Answers of no use to the driver, each queued just before the next
    /// answer of the same kind: another channel's, another GPADL's.
    pub stale: Vec<Vec<u8>>,
}

fn u32_at(p: &[u8], at: usize) -> u32 {
    u32::from_le_bytes(p[at..at + 4].try_into().unwrap())
}

/// A host answer of `kind` with its u32 fields from byte 8.
pub fn answer(kind: u32, fields: &[u32]) -> Vec<u8> {
    let mut a = vec![0u8; 8 + 4 * fields.len()];
    a[..4].copy_from_slice(&kind.to_le_bytes());
    for (i, f) in fields.iter().enumerate() {
        a[8 + 4 * i..12 + 4 * i].copy_from_slice(&f.to_le_bytes());
    }
    a
}

impl Model {
    pub fn new(script: Script) -> Self {
        Self {
            busy_left: script.busy,
            script,
            posted: Vec::new(),
            answers: VecDeque::new(),
            signals: 0,
            now: 0,
            gpadl_pfns: Vec::new(),
            gpadl_want: 0,
            gpadls: Vec::new(),
            open: None,
            refuse: false,
            stale: Vec::new(),
        }
    }

    fn take(&mut self, p: &[u8]) {
        match u32_at(p, 0) {
            8 => {
                let relid = u32_at(p, 8);
                let id = u32_at(p, 12);
                let buflen = u32_at(p, 16) & 0xFFFF;
                assert_eq!(u32_at(p, 16) >> 16, 1, "one range");
                let total = (buflen as usize - 8) / 8;
                assert_eq!(u32_at(p, 20) as usize, total * PAGE, "whole pages");
                assert_eq!(u32_at(p, 24), 0, "from the first page's start");
                self.gpadl_want = total;
                self.gpadl_pfns = p[28..]
                    .chunks(8)
                    .map(|c| u64::from_le_bytes(c.try_into().unwrap()))
                    .collect();
                self.finish_gpadl(relid, id);
            }
            9 => {
                let id = u32_at(p, 12);
                self.gpadl_pfns.extend(
                    p[16..]
                        .chunks(8)
                        .map(|c| u64::from_le_bytes(c.try_into().unwrap())),
                );
                self.finish_gpadl(u32::MAX, id);
            }
            5 => {
                assert_eq!(p.len(), 148, "the whole open");
                assert_eq!(u32_at(p, 20), 0, "VP 0");
                let (relid, open_id, gpadl, at) =
                    (u32_at(p, 8), u32_at(p, 12), u32_at(p, 16), u32_at(p, 24));
                assert!(
                    self.gpadls.iter().any(|(id, _)| *id == gpadl),
                    "open on a made GPADL"
                );
                if let Some(status) = self.script.open_status {
                    if status == 0 {
                        self.open = Some((relid, gpadl, at));
                    }
                    self.reply(answer(6, &[relid, open_id, status]));
                }
            }
            7 => self.open = None,
            11 => {
                let id = u32_at(p, 12);
                assert!(
                    self.open.is_none_or(|o| o.1 != id),
                    "teardown of an open channel's GPADL"
                );
                self.gpadls.retain(|(g, _)| *g != id);
                if self.script.torndown {
                    self.reply(answer(12, &[id]));
                }
            }
            k => panic!("a message a channel's driver never sends: {k}"),
        }
    }

    fn finish_gpadl(&mut self, relid: u32, id: u32) {
        if self.gpadl_pfns.len() < self.gpadl_want {
            return;
        }
        assert_eq!(
            self.gpadl_pfns.len(),
            self.gpadl_want,
            "no more pages than the header said"
        );
        let pfns = std::mem::take(&mut self.gpadl_pfns);
        let relid = if relid == u32::MAX {
            self.last_relid()
        } else {
            relid
        };
        if let Some(status) = self.script.gpadl_status {
            if status == 0 {
                self.gpadls.push((id, pfns));
            }
            self.reply(answer(10, &[relid, id, status]));
        }
    }

    fn reply(&mut self, a: Vec<u8>) {
        let kind = u32_at(&a, 0);
        let (before, rest): (Vec<_>, Vec<_>) = std::mem::take(&mut self.stale)
            .into_iter()
            .partition(|s| u32_at(s, 0) == kind);
        self.stale = rest;
        self.answers.extend(before);
        self.answers.push_back(a);
    }

    fn last_relid(&self) -> u32 {
        self.posted
            .iter()
            .rev()
            .find(|p| u32_at(p, 0) == 8)
            .map_or(0, |p| u32_at(p, 8))
    }

    pub fn kinds(&self) -> Vec<u32> {
        self.posted.iter().map(|p| u32_at(p, 0)).collect()
    }
}

impl Host for Model {
    fn post(&mut self, m: &[u8]) -> Result<(), PostError> {
        assert!(m.len() <= MESSAGE_MAX, "a message past a SynIC slot");
        if self.refuse {
            return Err(PostError::Refused);
        }
        if self.busy_left > 0 {
            self.busy_left -= 1;
            return Err(PostError::Busy);
        }
        self.busy_left = self.script.busy;
        self.posted.push(m.to_vec());
        self.take(m);
        Ok(())
    }

    fn answer(&mut self, out: &mut [u8; MESSAGE_MAX]) -> Option<usize> {
        let a = self.answers.pop_front()?;
        out[..a.len()].copy_from_slice(&a);
        Some(a.len())
    }

    fn signal(&mut self) {
        self.signals += 1;
    }

    fn now_ms(&mut self) -> u64 {
        self.now
    }

    fn idle(&mut self) {
        self.now += 10;
    }
}
