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

//! The channel messages a channel's driver posts, and the host's answers,
//! as QEMU's vmbus-proto.h lays them out: an 8-byte header (the type, then
//! padding), then the fields. A SynIC message carries at most 240 bytes.

pub const MESSAGE_MAX: usize = 240;

pub const OPENCHANNEL: u32 = 5;
pub const OPENCHANNEL_RESULT: u32 = 6;
pub const CLOSECHANNEL: u32 = 7;
pub const GPADL_HEADER: u32 = 8;
pub const GPADL_BODY: u32 = 9;
pub const GPADL_CREATED: u32 = 10;
pub const GPADL_TEARDOWN: u32 = 11;
pub const GPADL_TORNDOWN: u32 = 12;

/// GPADL_HEADER's fixed part: header, relid, id, range_buflen, rangecount,
/// then its one range's byte count and offset.
const HEADER_FIXED: usize = 28;
/// GPADL_BODY's: header, message number, id.
const BODY_FIXED: usize = 16;
/// Page numbers a header carries, and each body after it.
pub const HEADER_PFNS: usize = (MESSAGE_MAX - HEADER_FIXED) / 8;
pub const BODY_PFNS: usize = (MESSAGE_MAX - BODY_FIXED) / 8;

/// The open's user data (VMBUS_CHANNEL_USER_DATA_SIZE).
pub const USER_DATA: usize = 120;
const OPEN_BYTES: usize = 28 + USER_DATA;

/// A message built in place: its bytes and its length.
pub struct Msg {
    pub buf: [u8; MESSAGE_MAX],
    pub len: usize,
}

impl Msg {
    fn new(kind: u32, len: usize) -> Self {
        let mut buf = [0u8; MESSAGE_MAX];
        buf[..4].copy_from_slice(&kind.to_le_bytes());
        Self { buf, len }
    }

    fn put32(&mut self, at: usize, v: u32) {
        self.buf[at..at + 4].copy_from_slice(&v.to_le_bytes());
    }

    fn put64(&mut self, at: usize, v: u64) {
        self.buf[at..at + 8].copy_from_slice(&v.to_le_bytes());
    }

    pub fn bytes(&self) -> &[u8] {
        &self.buf[..self.len]
    }
}

/// GPADL_HEADER for `pfns`, all of one range of whole pages, with as many
/// of them as fit; the rest go in bodies (`gpadl_bodies`).
pub fn gpadl_header(relid: u32, id: u32, pfns: &[u64]) -> Option<Msg> {
    let total = u32::try_from(pfns.len()).ok()?;
    let range_buflen = u16::try_from(8 + 8 * pfns.len()).ok()?;
    let bytes = total.checked_mul(4096)?;
    let here = pfns.len().min(HEADER_PFNS);
    let mut m = Msg::new(GPADL_HEADER, HEADER_FIXED + 8 * here);
    m.put32(8, relid);
    m.put32(12, id);
    m.put32(16, u32::from(range_buflen) | 1 << 16);
    m.put32(20, bytes);
    m.put32(24, 0);
    for (i, p) in pfns[..here].iter().enumerate() {
        m.put64(HEADER_FIXED + 8 * i, *p);
    }
    Some(m)
}

/// The GPADL_BODY messages after the header, in order.
pub fn gpadl_bodies(id: u32, pfns: &[u64]) -> impl Iterator<Item = Msg> + '_ {
    let rest = pfns.get(HEADER_PFNS..).unwrap_or(&[]);
    rest.chunks(BODY_PFNS).enumerate().map(move |(n, chunk)| {
        let mut m = Msg::new(GPADL_BODY, BODY_FIXED + 8 * chunk.len());
        m.put32(8, n as u32);
        m.put32(12, id);
        for (i, p) in chunk.iter().enumerate() {
            m.put64(BODY_FIXED + 8 * i, *p);
        }
        m
    })
}

/// OPENCHANNEL: the rings in GPADL `gpadl`, the host's ring `in_at` pages
/// in, on VP 0, with the interface's user data.
pub fn open(relid: u32, open_id: u32, gpadl: u32, in_at: u32, user: &[u8]) -> Option<Msg> {
    if user.len() > USER_DATA {
        return None;
    }
    let mut m = Msg::new(OPENCHANNEL, OPEN_BYTES);
    m.put32(8, relid);
    m.put32(12, open_id);
    m.put32(16, gpadl);
    m.put32(20, 0);
    m.put32(24, in_at);
    m.buf[28..28 + user.len()].copy_from_slice(user);
    Some(m)
}

pub fn close(relid: u32) -> Msg {
    let mut m = Msg::new(CLOSECHANNEL, 12);
    m.put32(8, relid);
    m
}

pub fn gpadl_teardown(relid: u32, id: u32) -> Msg {
    let mut m = Msg::new(GPADL_TEARDOWN, 16);
    m.put32(8, relid);
    m.put32(12, id);
    m
}

/// A host answer for this channel.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Answer {
    Opened {
        relid: u32,
        open_id: u32,
        status: u32,
    },
    GpadlCreated {
        relid: u32,
        id: u32,
        status: u32,
    },
    GpadlTorndown {
        id: u32,
    },
    Other(u32),
}

fn u32_at(p: &[u8], at: usize) -> Option<u32> {
    Some(u32::from_le_bytes(p.get(at..at + 4)?.try_into().ok()?))
}

/// Read a host answer's payload, held to its length.
pub fn answer(p: &[u8]) -> Option<Answer> {
    let kind = u32_at(p, 0)?;
    Some(match kind {
        OPENCHANNEL_RESULT => Answer::Opened {
            relid: u32_at(p, 8)?,
            open_id: u32_at(p, 12)?,
            status: u32_at(p, 16)?,
        },
        GPADL_CREATED => Answer::GpadlCreated {
            relid: u32_at(p, 8)?,
            id: u32_at(p, 12)?,
            status: u32_at(p, 16)?,
        },
        GPADL_TORNDOWN => Answer::GpadlTorndown { id: u32_at(p, 8)? },
        k => Answer::Other(k),
    })
}
