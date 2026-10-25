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

//! The network VSP on the channel's rings, written from OpenVMM's netvsp
//! (vm/devices/net/netvsp/src/lib.rs, cited "netvsp:LINE"; buffers.rs
//! "buffers:LINE"; netvsp_protocol "proto:LINE", rndisprot.rs "rndis:LINE").
//! A packet netvsp would end the channel's worker for is recorded as a
//! violation and the server stops answering, as netvsp's worker stops.

use std::collections::VecDeque;

use nonos_vmbus::model::HostRing;

use super::wire::Wire;
use super::{Grants, PAGE, RINGS_PFN};

const INBAND: u16 = 6;
const XFER_PAGES: u16 = 7;
const COMP: u16 = 0xb;
/// netvsp:2400-2405.
pub const SUPPORTED: [u32; 6] = [0x2, 0x30002, 0x40000, 0x50000, 0x60000, 0x60001];
const V2: u32 = 0x30002;
/// proto:38: the message a completion is padded to, from 6.1.
const MESSAGE: usize = 0x28;
/// buffers:138-146, netvsp:3456-3458.
const RX_HEADER: u32 = 256;
const FOOTER: u32 = 36;
const DEFAULT_MTU: u32 = 1514;
const MAX_MTU: u32 = 9216;
/// netvsp:5285.
const SECTION: u32 = 6144;
/// rndis:30, 58, 72, 79.
const OK: u32 = 0;
const NOT_SUPPORTED: u32 = 0xC000_00BB;
const INVALID_DATA: u32 = 0xC001_0015;

/// A packet for the guest: type, flags, id, descriptor, data.
type Outgoing = (u16, u16, u64, Vec<u8>, Vec<u8>);

pub struct Netvsp {
    pub grants: Grants,
    pub wire: Wire,
    /// The address the host assigned the port.
    pub mac: [u8; 6],
    /// The guest's own address, taken from NetworkAddress (netvsp parses
    /// the value and ignores the name, netvsp:3931-3997; whether the switch
    /// then carries it is the wire's MAC spoofing setting).
    pub taken: Option<[u8; 6]>,
    pub versions: Vec<u32>,
    pub violations: Vec<&'static str>,
    pub version: Option<u32>,
    mtu: Option<u32>,
    pub ndis_version: Option<(u32, u32)>,
    recv: Option<(Vec<u64>, u32, u32)>,
    send: Option<Vec<u64>>,
    operational: bool,
    pub rndis_ready: bool,
    pub filter: u32,
    rx_free: Vec<bool>,
    rx_out: Vec<(u64, Vec<u32>)>,
    out: VecDeque<Outgoing>,
    /// What the guest sent: protocol message types, then RNDIS (type, OID).
    pub nvsp_seen: Vec<u32>,
    pub rndis_seen: Vec<(u32, u32)>,
    /// Frames waiting to reach the guest, and those dropped for want of a
    /// filter (netvsp:5617-5620).
    pub inbound: VecDeque<Vec<u8>>,
    pub filtered: u32,
    /// Every frame the guest sent, carried or not.
    pub tx: Vec<Vec<u8>>,
    /// Knobs: sends never completed; control messages never answered; a
    /// receive range past the buffer.
    pub withhold_tx: bool,
    pub mute: bool,
    pub bad_range: bool,
    /// The host reads nothing from the guest's ring.
    pub stalled: bool,
    /// Protocol messages taken and never completed.
    pub mute_nvsp: bool,
    /// The id of every request in band, in order.
    pub ids: Vec<u64>,
    /// Sends withheld, completed on `release_tx`.
    held: Vec<u64>,
    /// The receive buffer's section as the host reports it, in place of
    /// netvsp's own sizing; the link a stricter host reports at the start;
    /// the next control reply placed at the receive buffer's very end.
    pub recv_section: Option<(u32, u32)>,
    pub media_disconnected: bool,
    pub at_end: bool,
}

fn word(b: &[u8], at: usize) -> u32 {
    u32::from_le_bytes(b[at..at + 4].try_into().unwrap())
}

fn message(ty: u32, words: &[u32]) -> Vec<u8> {
    let mut m = vec![0u8; MESSAGE];
    m[..4].copy_from_slice(&ty.to_le_bytes());
    for (i, w) in words.iter().enumerate() {
        m[4 + 4 * i..8 + 4 * i].copy_from_slice(&w.to_le_bytes());
    }
    m
}

fn rndis(ty: u32, words: &[u32], tail: &[u8]) -> Vec<u8> {
    let len = 8 + 4 * words.len() + tail.len();
    let mut m = Vec::with_capacity(len);
    m.extend_from_slice(&ty.to_le_bytes());
    m.extend_from_slice(&(len as u32).to_le_bytes());
    for w in words {
        m.extend_from_slice(&w.to_le_bytes());
    }
    m.extend_from_slice(tail);
    m
}

impl Netvsp {
    pub fn new(grants: Grants) -> Self {
        Self {
            grants,
            wire: Wire::new(),
            mac: [0x00, 0x15, 0x5D, 0x01, 0x02, 0x03],
            taken: None,
            versions: SUPPORTED.to_vec(),
            violations: Vec::new(),
            version: None,
            mtu: None,
            ndis_version: None,
            recv: None,
            send: None,
            operational: false,
            rndis_ready: false,
            filter: 0,
            rx_free: Vec::new(),
            rx_out: Vec::new(),
            out: VecDeque::new(),
            nvsp_seen: Vec::new(),
            rndis_seen: Vec::new(),
            inbound: VecDeque::new(),
            filtered: 0,
            tx: Vec::new(),
            withhold_tx: false,
            mute: false,
            bad_range: false,
            stalled: false,
            mute_nvsp: false,
            ids: Vec::new(),
            held: Vec::new(),
            recv_section: None,
            media_disconnected: false,
            at_end: false,
        }
    }

    /// Complete every withheld send, and withhold no more.
    pub fn release_tx(&mut self) {
        self.withhold_tx = false;
        for txid in std::mem::take(&mut self.held) {
            self.complete(txid, message(108, &[1]));
        }
    }

    fn violate(&mut self, why: &'static str) {
        self.violations.push(why);
    }

    /// Everything waiting on the guest's ring taken and answered, then the
    /// frames waiting for the guest delivered.
    pub fn run(&mut self, gpadls: &[(u32, Vec<u64>)], ring_gpadl: u32, in_at: usize, _now: u64) {
        if !self.violations.is_empty() || self.stalled {
            return;
        }
        let pfns = &gpadls.iter().find(|(id, _)| *id == ring_gpadl).unwrap().1;
        assert_eq!(pfns[0], RINGS_PFN, "the rings are the first grant");
        let total = pfns.len();
        let mut m = self.grants.pages(RINGS_PFN);
        let guest = HostRing { hdr: 0, len: ((in_at - 1) * PAGE) as u32 };
        let host = HostRing { hdr: in_at * PAGE, len: ((total - in_at - 1) * PAGE) as u32 };
        loop {
            self.flush(&host, &mut m);
            let Some((ty, flags, txid, desc, data)) = guest.recv(&mut m) else { break };
            self.take(gpadls, ty, flags, txid, &desc, &data);
            if !self.violations.is_empty() {
                return;
            }
        }
        self.deliver();
        self.flush(&host, &mut m);
    }

    fn flush(&mut self, host: &HostRing, m: &mut nonos_vmbus::Pages) {
        while let Some((ty, flags, txid, desc, data)) = self.out.front() {
            if host.send_packet(m, *ty, *flags, *txid, desc, data).is_none() {
                break;
            }
            self.out.pop_front();
        }
    }

    fn complete(&mut self, txid: u64, data: Vec<u8>) {
        self.out.push_back((COMP, 0, txid, Vec::new(), data));
    }

    fn take(&mut self, gpadls: &[(u32, Vec<u64>)], ty: u16, flags: u16, txid: u64, desc: &[u8], data: &[u8]) {
        if ty == COMP {
            // netvsp:5804-5817: a receive's completion frees its ranges.
            match self.rx_out.iter().position(|(t, _)| *t == txid) {
                Some(i) => {
                    let (_, ids) = self.rx_out.remove(i);
                    for id in ids {
                        self.rx_free[id as usize] = true;
                    }
                }
                None => self.violate("completion of no receive"),
            }
            return;
        }
        if ty != INBAND || flags & 1 == 0 || !desc.is_empty() {
            return self.violate("a request that is not in band with a completion asked");
        }
        let ty = word(data, 0);
        self.nvsp_seen.push(ty);
        self.ids.push(txid);
        if self.mute_nvsp && ty != 107 {
            return;
        }
        if self.version.is_none() {
            // netvsp:5313-5345: nothing but INIT before a version.
            if ty != 1 {
                return self.violate("a message before INIT");
            }
            let asked = word(data, 4);
            let ok = self.versions.contains(&asked);
            if ok {
                self.version = Some(asked);
            }
            let deprecated = if asked == 2 && ok { 2 } else { u32::MAX };
            return self.complete(txid, message(2, &[deprecated, 34, u32::from(ok)]));
        }
        if !self.operational {
            return self.initializing(gpadls, ty, txid, data);
        }
        match ty {
            107 => self.rndis_packet(txid, data),
            _ => self.violate("a message the running channel never takes"),
        }
    }

    /// netvsp:5134-5139.
    fn ready(&self) -> bool {
        (self.mtu.is_some() || self.version < Some(V2)) && self.ndis_version.is_some() && self.recv.is_some()
    }

    /// netvsp:5192-5310: the buffers, in any order but the MTU before the
    /// receive buffer from version 2.
    fn initializing(&mut self, gpadls: &[(u32, Vec<u64>)], ty: u32, txid: u64, data: &[u8]) {
        match ty {
            125 => {
                if self.mtu.is_some() {
                    return self.violate("NDIS config twice");
                }
                let mtu = word(data, 4);
                self.mtu = Some(if (DEFAULT_MTU..=MAX_MTU).contains(&mtu) { mtu } else { DEFAULT_MTU });
                self.complete(txid, Vec::new());
            }
            100 => {
                if self.ndis_version.is_some() {
                    return self.violate("NDIS version twice");
                }
                self.ndis_version = Some((word(data, 4), word(data, 8)));
                self.complete(txid, Vec::new());
            }
            101 => {
                if self.recv.is_some() {
                    return self.violate("receive buffer twice");
                }
                let mtu = match self.mtu {
                    Some(m) => m,
                    None if self.version < Some(V2) => DEFAULT_MTU,
                    None => return self.violate("receive buffer before the MTU"),
                };
                let sub = RX_HEADER + mtu + FOOTER;
                let Some((_, pfns)) = gpadls.iter().find(|(id, _)| *id == word(data, 4)) else {
                    return self.violate("receive buffer on no GPADL");
                };
                let count = (pfns.len() * PAGE) as u32 / sub;
                if count == 0 {
                    return self.violate("receive buffer too small");
                }
                let (sub, count) = self.recv_section.unwrap_or((sub, count));
                self.recv = Some((pfns.clone(), sub, count));
                self.rx_free = vec![true; count as usize];
                self.complete(txid, message(102, &[1, 1, 0, sub, count, sub * count]));
            }
            104 => {
                if self.send.is_some() {
                    return self.violate("send buffer twice");
                }
                let Some((_, pfns)) = gpadls.iter().find(|(id, _)| *id == word(data, 4)) else {
                    return self.violate("send buffer on no GPADL");
                };
                self.send = Some(pfns.clone());
                self.complete(txid, message(105, &[1, SECTION]));
            }
            107 => {
                // netvsp:5293-5306: RNDIS once the rest is in place.
                if !self.ready() {
                    return self.violate("RNDIS before the buffers");
                }
                self.operational = true;
                self.rndis_packet(txid, data);
            }
            _ => self.violate("a message the start never takes"),
        }
    }

    /// netvsp:2275-2291, 5784-5802, 5975-6019.
    fn rndis_packet(&mut self, txid: u64, data: &[u8]) {
        let (channel, section, size) = (word(data, 4), word(data, 8), word(data, 12));
        if section == u32::MAX {
            return self.violate("a message by page: this model takes the send buffer's");
        }
        let Some(pfns) = self.send.clone() else { return self.violate("a section with no send buffer") };
        if (section as usize + 1) * SECTION as usize > pfns.len() * PAGE || size > SECTION {
            return self.violate("a section past the send buffer");
        }
        let m = self.grants.read(&pfns, section as usize * SECTION as usize, size as usize);
        let ty = word(&m, 0);
        if self.mute && ty != 1 {
            return;
        }
        let status = if ty == 1 {
            if channel != 0 {
                self.violate("a frame on the control channel");
            }
            self.frame_out(&m)
        } else {
            if channel != 1 {
                self.violate("a control message on the data channel");
            }
            self.control(&m);
            true
        };
        if self.withhold_tx {
            self.held.push(txid);
        } else {
            self.complete(txid, message(108, &[if status { 1 } else { 2 }]));
        }
    }

    /// netvsp:2601-2640: no out-of-band data, no VC, data within the message.
    fn frame_out(&mut self, m: &[u8]) -> bool {
        let p = &m[8..];
        let (off, len) = (word(p, 0) as usize, word(p, 4) as usize);
        if word(p, 8) != 0 || word(p, 12) != 0 || word(p, 16) != 0 || word(p, 28) != 0 {
            return false;
        }
        if len == 0 || off + len > p.len() {
            return false;
        }
        let frame = p[off..off + len].to_vec();
        self.tx.push(frame.clone());
        if self.wire.carries_out(&frame, self.mac, self.taken) {
            for back in self.wire.answer(&frame) {
                if self.wire.carries_in(&back, self.mac, self.taken) {
                    self.inbound.push_back(back);
                }
            }
        }
        true
    }

    /// netvsp:3083-3249.
    fn control(&mut self, m: &[u8]) {
        let (ty, request) = (word(m, 0), word(m, 8));
        let oid = if ty == 4 || ty == 5 { word(m, 12) } else { 0 };
        self.rndis_seen.push((ty, oid));
        let reply = match ty {
            2 => {
                if self.rndis_ready {
                    return self.violate("INITIALIZE twice");
                }
                self.rndis_ready = true;
                // netvsp:3097-3112.
                rndis(0x8000_0002, &[request, OK, 1, 0, 1, 0, 8, 0xEFFF_FFFF, 3, 0, 0], &[])
            }
            4 => {
                let info: Vec<u8> = match oid {
                    // netvsp:3627-3630.
                    0x0101_0101 | 0x0101_0102 => self.mac.to_vec(),
                    // netvsp:3618-3620: always connected.
                    0x0001_0114 => u32::from(self.media_disconnected).to_le_bytes().to_vec(),
                    _ => Vec::new(),
                };
                let status = if info.is_empty() { NOT_SUPPORTED } else { OK };
                // netvsp:3165-3178: the information right after the reply.
                rndis(0x8000_0004, &[request, status, info.len() as u32, 16], &info)
            }
            5 => {
                let info = &m[28..];
                let status = match oid {
                    // netvsp:3738-3740, 3831-3838.
                    0x0001_010E => {
                        self.filter = word(info, 0);
                        OK
                    }
                    0x0001_021B => self.config_parameter(info),
                    _ => NOT_SUPPORTED,
                };
                rndis(0x8000_0005, &[request, status], &[])
            }
            _ => return self.violate("an RNDIS message netvsp does not implement"),
        };
        if let Some(id) = self.rx_take() {
            self.indicate(1, &[(id, reply)]);
        }
    }

    /// netvsp:3931-3997: lengths at most 255; a string's first character
    /// must be a digit; the name is matched against the offload settings
    /// alone, so NetworkAddress is taken and changes nothing in netvsp.
    fn config_parameter(&mut self, info: &[u8]) -> u32 {
        let (name_at, name_len, ty, value_at, value_len) =
            (word(info, 0) as usize, word(info, 4) as usize, word(info, 8), word(info, 12) as usize, word(info, 16) as usize);
        if name_len > 255 || value_len > 255 {
            return INVALID_DATA;
        }
        let utf16 = |at: usize, len: usize| -> Option<String> {
            let units: Vec<u16> =
                info.get(at..at + len)?.chunks(2).map(|c| u16::from_le_bytes([c[0], c[1]])).collect();
            String::from_utf16(&units).ok()
        };
        let (Some(name), Some(value)) = (utf16(name_at, name_len), utf16(value_at, value_len)) else {
            return INVALID_DATA;
        };
        if ty == 2 && !value.as_bytes().first().is_some_and(u8::is_ascii_digit) {
            return INVALID_DATA;
        }
        if name == "NetworkAddress" && value.len() == 12 {
            let b: Vec<u8> = (0..6).filter_map(|i| u8::from_str_radix(&value[2 * i..2 * i + 2], 16).ok()).collect();
            self.taken = b.try_into().ok();
        }
        OK
    }

    fn rx_take(&mut self) -> Option<u32> {
        let i = self.rx_free.iter().position(|f| *f)?;
        self.rx_free[i] = false;
        Some(i as u32)
    }

    /// One transfer-pages packet: each (id, message) written at its
    /// sub-allocation, the ranges named, the packet's id the first's
    /// (netvsp:3254-3291, 5625-5645).
    fn indicate(&mut self, channel: u32, msgs: &[(u32, Vec<u8>)]) {
        let (pfns, sub, _) = self.recv.clone().unwrap();
        let mut desc = vec![0u8; 8];
        desc[0..2].copy_from_slice(&0xCAFEu16.to_le_bytes());
        desc[4..8].copy_from_slice(&(msgs.len() as u32).to_le_bytes());
        for (id, m) in msgs {
            self.grants.write(&pfns, (id * sub) as usize, m);
            // Data messages claim the whole sub-allocation (buffers:270-275);
            // control messages their own length.
            let len = if channel == 0 { sub } else { m.len() as u32 };
            let end = pfns.len() as u32 * PAGE as u32;
            let at = if self.bad_range {
                end
            } else if std::mem::take(&mut self.at_end) {
                self.grants.write(&pfns, (end - len) as usize, m);
                end - len
            } else {
                id * sub
            };
            desc.extend_from_slice(&len.to_le_bytes());
            desc.extend_from_slice(&at.to_le_bytes());
        }
        let txid = u64::from(msgs[0].0);
        self.rx_out.push((txid, msgs.iter().map(|(id, _)| *id).collect()));
        let nvsp = message(107, &[channel, u32::MAX, 0]);
        self.out.push_back((XFER_PAGES, 1, txid, desc, nvsp));
    }

    /// A link change, as an indication on the control channel
    /// (netvsp:5410-5430, 3304-3330).
    pub fn indicate_link(&mut self, up: bool) {
        if let Some(id) = self.rx_take() {
            let status = if up { 0x4001_000B } else { 0x4001_000C };
            self.indicate(1, &[(id, rndis(7, &[status, 0, 0], &[]))]);
        }
    }

    /// Frames for the guest, a few to a packet, each behind its RNDIS
    /// header and one per-packet info, the frame at 256 (buffers:203-301).
    fn deliver(&mut self) {
        if !self.operational || self.recv.is_none() {
            return;
        }
        if self.filter == 0 {
            self.filtered += self.inbound.len() as u32;
            self.inbound.clear();
            return;
        }
        let sub = self.recv.as_ref().unwrap().1;
        while !self.inbound.is_empty() {
            let mut batch = Vec::new();
            while batch.len() < 4 {
                let Some(id) = self.rx_take() else { break };
                let Some(f) = self.inbound.pop_front() else {
                    self.rx_free[id as usize] = true;
                    break;
                };
                let mut m = rndis(1, &[RX_HEADER - 8, f.len() as u32, 0, 0, 0, 36, 16, 0, 0], &[]);
                m.extend_from_slice(&16u32.to_le_bytes());
                m.extend_from_slice(&0u32.to_le_bytes());
                m.extend_from_slice(&12u32.to_le_bytes());
                m.extend_from_slice(&0u32.to_le_bytes());
                m[4..8].copy_from_slice(&sub.to_le_bytes());
                m.resize(RX_HEADER as usize, 0);
                m.extend_from_slice(&f);
                batch.push((id, m));
            }
            if batch.is_empty() {
                return;
            }
            self.indicate(0, &batch);
        }
    }
}
