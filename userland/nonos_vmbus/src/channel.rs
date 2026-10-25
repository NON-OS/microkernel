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

//! A channel opened and closed, and its packets sent and taken. The shared
//! pages hold the guest's ring first (`out`, which the host reads), then
//! the host's (`inb`, which the guest reads). One GPADL hands them all to
//! the host, and the open names where the host's ring starts. This is
//! Linux's vmbus_open and vmbus_close, and what QEMU's vmbus.c answers.
//!
//! Every step waits a bounded time for its answer. A step that fails
//! undoes the ones before it: a GPADL made for an open that failed is torn
//! down again, so the host keeps no page of a channel that did not open.

use crate::mem::Shared;
use crate::msg::{self, Answer, MESSAGE_MAX};
use crate::packet::Header;
use crate::ring::{Ring, RingError, PAGE};

/// How long the host has for each answer.
pub const ANSWER_MS: u64 = 5_000;

/// A post the kernel did not take.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PostError {
    /// No room at the host now; try again.
    Busy,
    /// Refused outright.
    Refused,
}

/// The channel's line to the host through the kernel: its record's posts,
/// answers, signal and clock.
pub trait Host {
    fn post(&mut self, m: &[u8]) -> Result<(), PostError>;
    /// The oldest host answer for this channel, its payload into `out`.
    fn answer(&mut self, out: &mut [u8; MESSAGE_MAX]) -> Option<usize>;
    fn signal(&mut self);
    fn now_ms(&mut self) -> u64;
    /// Wait a little before looking again.
    fn idle(&mut self);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OpenError {
    /// The rings do not fit the shared pages, or a message would not encode.
    Layout,
    /// The kernel refused a message.
    Refused,
    /// The host refused the GPADL, with its status.
    GpadlRefused(u32),
    /// The host refused the open, with its status.
    OpenRefused(u32),
    /// The host did not answer in time.
    Silent,
}

/// Where the rings are: `out_pages` from page 0, then `in_pages`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Layout {
    pub out_pages: u32,
    pub in_pages: u32,
}

impl Layout {
    pub fn pages(&self) -> usize {
        (self.out_pages + self.in_pages) as usize
    }

    fn rings(&self) -> (Ring, Ring) {
        let out = Ring::new(0, self.out_pages as usize);
        let inb = Ring::new(self.out_pages as usize * PAGE, self.in_pages as usize);
        (out, inb)
    }
}

/// An open channel.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Channel {
    pub relid: u32,
    pub gpadl: u32,
    pub out: Ring,
    pub inb: Ring,
}

fn post(h: &mut impl Host, m: &[u8]) -> Result<(), OpenError> {
    let until = h.now_ms().saturating_add(ANSWER_MS);
    loop {
        match h.post(m) {
            Ok(()) => return Ok(()),
            Err(PostError::Refused) => return Err(OpenError::Refused),
            Err(PostError::Busy) if h.now_ms() >= until => return Err(OpenError::Silent),
            Err(PostError::Busy) => h.idle(),
        }
    }
}

/// The first answer `want` accepts, others dropped, within ANSWER_MS.
fn wait<T>(h: &mut impl Host, mut want: impl FnMut(Answer) -> Option<T>) -> Option<T> {
    let until = h.now_ms().saturating_add(ANSWER_MS);
    let mut buf = [0u8; MESSAGE_MAX];
    loop {
        while let Some(n) = h.answer(&mut buf) {
            if let Some(t) = msg::answer(&buf[..n.min(MESSAGE_MAX)]).and_then(&mut want) {
                return Some(t);
            }
        }
        if h.now_ms() >= until {
            return None;
        }
        h.idle();
    }
}

/// Hand the host GPADL `id` over the page numbers `pfns`, for channel
/// `relid`, and wait for it: the rings' at the open, and any buffer a driver
/// shares beside them (netvsc's receive and send buffers). One the host
/// never answers is torn down again, so the host keeps no page of it.
pub fn gpadl(h: &mut impl Host, relid: u32, id: u32, pfns: &[u64]) -> Result<(), OpenError> {
    post(
        h,
        msg::gpadl_header(relid, id, pfns)
            .ok_or(OpenError::Layout)?
            .bytes(),
    )?;
    for body in msg::gpadl_bodies(id, pfns) {
        post(h, body.bytes())?;
    }
    let created = wait(h, |a| match a {
        Answer::GpadlCreated {
            relid: r,
            id: i,
            status,
        } if (r, i) == (relid, id) => Some(status),
        _ => None,
    });
    match created {
        Some(0) => Ok(()),
        Some(status) => Err(OpenError::GpadlRefused(status)),
        // The host may make it yet: tear it down, so it holds no page.
        None => {
            let _ = teardown(h, relid, id);
            Err(OpenError::Silent)
        }
    }
}

/// Tear GPADL `id` down and wait for the host to say it is gone; until
/// then the host may still reach its pages.
pub fn teardown(h: &mut impl Host, relid: u32, id: u32) -> Result<(), OpenError> {
    post(h, msg::gpadl_teardown(relid, id).bytes())?;
    let gone = wait(h, |a| match a {
        Answer::GpadlTorndown { id: i } if i == id => Some(()),
        _ => None,
    });
    gone.ok_or(OpenError::Silent)
}

/// Open channel `relid` on the shared pages `m`, whose first page number is
/// `first_pfn`: the rings set empty, the GPADL `gpadl` made over every
/// page, then the open with `user` data.
pub fn open(
    h: &mut impl Host,
    m: &mut impl Shared,
    relid: u32,
    gpadl: u32,
    first_pfn: u64,
    layout: Layout,
    user: &[u8],
) -> Result<Channel, OpenError> {
    if layout.out_pages < 2 || layout.in_pages < 2 || layout.pages() * PAGE > m.len() {
        return Err(OpenError::Layout);
    }
    let (out, inb) = layout.rings();
    out.init(m);
    inb.init(m);
    let mut pfns = [0u64; 512];
    let pfns = pfns.get_mut(..layout.pages()).ok_or(OpenError::Layout)?;
    for (i, p) in pfns.iter_mut().enumerate() {
        *p = first_pfn.checked_add(i as u64).ok_or(OpenError::Layout)?;
    }
    self::gpadl(h, relid, gpadl, pfns)?;
    let opened = msg::open(relid, relid, gpadl, layout.out_pages, user)
        .ok_or(OpenError::Layout)
        .and_then(|o| post(h, o.bytes()))
        .map(|()| {
            wait(h, |a| match a {
                Answer::Opened {
                    relid: r,
                    open_id,
                    status,
                } if (r, open_id) == (relid, relid) => Some(status),
                _ => None,
            })
        });
    match opened {
        Ok(Some(0)) => Ok(Channel {
            relid,
            gpadl,
            out,
            inb,
        }),
        Ok(answer) => {
            // Refused, or silent: the host may yet open it, so close first.
            if answer.is_none() {
                let _ = post(h, msg::close(relid).bytes());
            }
            let _ = teardown(h, relid, gpadl);
            Err(answer.map_or(OpenError::Silent, OpenError::OpenRefused))
        }
        Err(e) => {
            let _ = teardown(h, relid, gpadl);
            Err(e)
        }
    }
}

impl Channel {
    /// Close the channel and take its GPADL back. Ok once the host says the
    /// GPADL is gone; until then it may still write the pages.
    pub fn close(&self, h: &mut impl Host) -> Result<(), OpenError> {
        post(h, msg::close(self.relid).bytes())?;
        teardown(h, self.relid, self.gpadl)
    }

    /// Send one packet, signalling the host when it may be waiting.
    pub fn send(
        &self,
        h: &mut impl Host,
        m: &mut impl Shared,
        header: &Header,
        desc: &[u8],
        data: &[&[u8]],
    ) -> Result<(), RingError> {
        if self.out.send(m, header, desc, data)? {
            h.signal();
        }
        Ok(())
    }

    /// Take one packet the host sent into `out`, signalling it when it
    /// waits for the room this made.
    pub fn recv(
        &self,
        h: &mut impl Host,
        m: &mut impl Shared,
        out: &mut [u8],
    ) -> Result<Option<(Header, usize)>, RingError> {
        let got = self.inb.recv(m, out)?;
        Ok(got.map(|(hdr, n, signal)| {
            if signal {
                h.signal();
            }
            (hdr, n)
        }))
    }
}
