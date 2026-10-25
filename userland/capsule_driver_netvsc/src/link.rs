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

//! The channel to the network VSP: its messages and their completions, the
//! RNDIS messages the driver sends through the send buffer, and what the
//! host puts in the receive buffer. Every receive is a transfer-pages
//! packet naming ranges of the receive buffer (netvsp lib.rs:3262-3291,
//! 5630-5645); the driver reads each range's RNDIS message, then completes
//! the packet so the host may use those ranges again (lib.rs:5804-5817).
//! A completion the ring has no room for is owed, and sent first next time.

use alloc::collections::VecDeque;
use alloc::vec;
use alloc::vec::Vec;

use nonos_vmbus::packet::{
    xfer_pages, FLAG_REQUEST_COMPLETION, PACKET_COMP, PACKET_DATA_INBAND, PACKET_DATA_USING_XFER_PAGES,
};
use nonos_vmbus::{Channel, Header, Host, RingError, Shared};

use crate::constants::layout::MAX_SECTIONS;
use crate::constants::nvsp::{CONTROL_CHANNEL, DATA_CHANNEL};
use crate::nvsp::{self, Message};
use crate::rndis::{self, Received};

/// The largest packet the host sends: 375 ranges at most in one receive
/// (netvsp lib.rs:1110), each 8 bytes, beside the headers and the message.
const IN_MAX: usize = 4096;
/// A send's id is its section's, above every request's.
const TX_BASE: u64 = 1 << 48;
/// Frames waiting for the stack; one more is dropped and counted.
const FRAMES_MAX: usize = 64;
const ETH_MIN: usize = 14;
const ETH_MAX: usize = 1514;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LinkError {
    /// The host did not answer in time.
    Silent,
    /// No send buffer section came free in time.
    Busy,
    /// The host broke its ring; nothing more is sent.
    Broken,
    /// A message the protocol cannot carry.
    TooBig,
}

/// The shared buffers beside the rings, as the host was given them.
pub struct Buffers<M: Shared> {
    pub recv: M,
    pub send: M,
    /// The send buffer's section size, from the host (proto:341-350).
    pub section: usize,
    /// How many sections the send buffer holds, at most MAX_SECTIONS.
    pub sections: usize,
}

pub struct Link<H: Host, M: Shared> {
    pub host: H,
    pub rings: M,
    pub chan: Channel,
    pub bufs: Buffers<M>,
    /// Sections handed to the host and not yet completed, one bit each.
    busy: u64,
    next_id: u64,
    /// A protocol message's completion, by its id.
    answer: Option<(u64, Vec<u8>)>,
    /// RNDIS replies by request id, the latest few.
    replies: VecDeque<(u32, u32, Vec<u8>)>,
    pub frames: VecDeque<Vec<u8>>,
    pub dropped: u64,
    /// The link as the host last said it, if it did.
    pub link: Option<bool>,
    owed: VecDeque<u64>,
    pub dead: bool,
}

impl<H: Host, M: Shared> Link<H, M> {
    pub fn new(host: H, rings: M, chan: Channel, bufs: Buffers<M>) -> Self {
        Self {
            host,
            rings,
            chan,
            bufs,
            busy: 0,
            next_id: 0,
            answer: None,
            replies: VecDeque::new(),
            frames: VecDeque::new(),
            dropped: 0,
            link: None,
            owed: VecDeque::new(),
            dead: false,
        }
    }

    fn send_packet(&mut self, ty: u16, flags: u16, id: u64, data: &[u8]) -> Result<(), LinkError> {
        if self.dead {
            return Err(LinkError::Broken);
        }
        let header = Header::new(ty, flags, id);
        match self.chan.send(&mut self.host, &mut self.rings, &header, &[], &[data]) {
            Ok(()) => Ok(()),
            Err(RingError::Full) => Err(LinkError::Busy),
            Err(RingError::TooBig) => Err(LinkError::TooBig),
            Err(_) => {
                self.dead = true;
                Err(LinkError::Broken)
            }
        }
    }

    /// Take everything the host sent. Owed completions go first.
    pub fn service(&mut self) -> Result<(), LinkError> {
        match self.flush_owed() {
            Ok(()) | Err(LinkError::Busy) => {}
            Err(e) => return Err(e),
        }
        let mut buf = vec![0u8; IN_MAX];
        loop {
            if self.dead {
                return Err(LinkError::Broken);
            }
            match self.chan.recv(&mut self.host, &mut self.rings, &mut buf) {
                Ok(Some((h, n))) => self.take(&h, &buf[..n]),
                Ok(None) => return Ok(()),
                Err(RingError::TooBig) => {
                    if self.chan.inb.skip(&mut self.rings).is_err() {
                        self.dead = true;
                    }
                }
                Err(_) => self.dead = true,
            }
        }
    }

    fn take(&mut self, h: &Header, p: &[u8]) {
        let at = h.data_at.min(p.len());
        let (desc, body) = (&p[16.min(at)..at], &p[at..]);
        match h.ty {
            PACKET_COMP => {
                let id = h.transaction_id;
                if (TX_BASE..TX_BASE + MAX_SECTIONS as u64).contains(&id) {
                    self.busy &= !(1u64 << (id - TX_BASE));
                } else {
                    self.answer = Some((id, body.to_vec()));
                }
            }
            PACKET_DATA_USING_XFER_PAGES => {
                if nvsp::received(body).is_some() {
                    if let Some((_, ranges)) = xfer_pages(desc) {
                        for (count, offset) in ranges {
                            self.received(offset as usize, count as usize);
                        }
                    }
                }
                // Completed whatever it held, so the host has its ranges back.
                if h.flags & FLAG_REQUEST_COMPLETION != 0 {
                    self.owed.push_back(h.transaction_id);
                    let _ = self.flush_owed();
                }
            }
            _ => {}
        }
    }

    fn flush_owed(&mut self) -> Result<(), LinkError> {
        while let Some(&id) = self.owed.front() {
            self.send_packet(PACKET_COMP, 0, id, &nvsp::rndis_done())?;
            self.owed.pop_front();
        }
        Ok(())
    }

    /// One range of the receive buffer: an RNDIS message, copied out of the
    /// shared pages, then read. A range past the buffer is the host's error
    /// and is passed over.
    fn received(&mut self, offset: usize, count: usize) {
        let Some(end) = offset.checked_add(count) else { return };
        if end > self.bufs.recv.len() || count < 8 {
            return;
        }
        let mut m = vec![0u8; count];
        self.bufs.recv.read(offset, &mut m);
        match rndis::read(&m) {
            Some(Received::Reply { request, status, info: (at, n) }) => {
                if self.replies.len() == 8 {
                    self.replies.pop_front();
                }
                self.replies.push_back((request, status, m[at..at + n].to_vec()));
            }
            Some(Received::Frame(at, n)) if (ETH_MIN..=ETH_MAX).contains(&n) => {
                if self.frames.len() < FRAMES_MAX {
                    self.frames.push_back(m[at..at + n].to_vec());
                } else {
                    self.dropped += 1;
                }
            }
            Some(Received::Link(up)) => self.link = Some(up),
            _ => {}
        }
    }

    fn take_id(&mut self) -> u64 {
        self.next_id = if self.next_id + 1 >= TX_BASE { 1 } else { self.next_id + 1 };
        self.next_id
    }

    /// A protocol message and its completion's bytes, within `ms`.
    pub fn call(&mut self, m: &Message, ms: u64) -> Result<Vec<u8>, LinkError> {
        let id = self.take_id();
        let until = self.host.now_ms().saturating_add(ms);
        loop {
            match self.send_packet(PACKET_DATA_INBAND, FLAG_REQUEST_COMPLETION, id, m) {
                Ok(()) => break,
                Err(LinkError::Busy) if self.host.now_ms() < until => self.host.idle(),
                Err(LinkError::Busy) => return Err(LinkError::Silent),
                Err(e) => return Err(e),
            }
        }
        loop {
            self.service()?;
            if let Some((got, _)) = &self.answer {
                if *got == id {
                    return Ok(self.answer.take().map(|(_, b)| b).unwrap_or_default());
                }
            }
            if self.host.now_ms() >= until {
                return Err(LinkError::Silent);
            }
            self.host.idle();
        }
    }

    /// A free send buffer section, waited for until `until`.
    fn section(&mut self, until: u64) -> Result<usize, LinkError> {
        loop {
            let free = (0..self.bufs.sections).find(|i| self.busy & (1u64 << i) == 0);
            if let Some(i) = free {
                return Ok(i);
            }
            self.service()?;
            if self.host.now_ms() >= until {
                return Err(LinkError::Busy);
            }
            self.host.idle();
        }
    }

    /// One RNDIS message, `parts` back to back, through a send buffer
    /// section on `channel`. The section is the host's until it completes.
    pub fn send_rndis(&mut self, channel: u32, parts: &[&[u8]], ms: u64) -> Result<(), LinkError> {
        let len: usize = parts.iter().map(|p| p.len()).sum();
        if len > self.bufs.section {
            return Err(LinkError::TooBig);
        }
        let until = self.host.now_ms().saturating_add(ms);
        let i = self.section(until)?;
        let mut at = i * self.bufs.section;
        for p in parts {
            self.bufs.send.write(at, p);
            at += p.len();
        }
        self.bufs.send.barrier();
        let m = nvsp::rndis_packet(channel, i as u32, len as u32);
        self.busy |= 1u64 << i;
        loop {
            match self.send_packet(PACKET_DATA_INBAND, FLAG_REQUEST_COMPLETION, TX_BASE + i as u64, &m) {
                Ok(()) => return Ok(()),
                Err(LinkError::Busy) if self.host.now_ms() < until => {
                    self.service()?;
                    self.host.idle();
                }
                Err(e) => {
                    self.busy &= !(1u64 << i);
                    return Err(if e == LinkError::Busy { LinkError::Silent } else { e });
                }
            }
        }
    }

    /// An RNDIS request with id `request` on the control channel, and its
    /// reply's status and information buffer, within `ms`.
    pub fn control(&mut self, req: &[u8], request: u32, ms: u64) -> Result<(u32, Vec<u8>), LinkError> {
        let until = self.host.now_ms().saturating_add(ms);
        self.replies.retain(|r| r.0 != request);
        self.send_rndis(CONTROL_CHANNEL, &[req], ms)?;
        loop {
            self.service()?;
            if let Some(i) = self.replies.iter().position(|r| r.0 == request) {
                let (_, status, info) = self.replies.remove(i).unwrap();
                return Ok((status, info));
            }
            if self.host.now_ms() >= until {
                return Err(LinkError::Silent);
            }
            self.host.idle();
        }
    }

    /// One Ethernet frame on the data channel, behind its RNDIS header.
    pub fn send_frame(&mut self, frame: &[u8], ms: u64) -> Result<(), LinkError> {
        let header = rndis::packet_header(frame.len());
        self.send_rndis(DATA_CHANNEL, &[&header, frame], ms)
    }
}
