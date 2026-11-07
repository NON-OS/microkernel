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

//! The keyring's four lock operations, as the keyring, the shell and setup
//! frame them. A request is the keyring's header (`seq u32 | op u16 |
//! 0 u16`) and a body; a reply is `seq u32 | status i32` and a body.
//!
//! | op | body | answer |
//! |---|---|---|
//! | `OP_LOCK_SET` 32 | `pid u32 | cur_len u16 | new_len u16 | cur | new` | 0 + blob; `E_WRONG` + count + blob; `E_WAIT` + count |
//! | `OP_LOCK_CHECK` 33 | `pid u32 | len u16 | passphrase` | 0 + blob; `E_WRONG` + count + blob; `E_WAIT` + count; `E_NOENT` |
//! | `OP_LOCK_LOAD` 34 | `pid u32 | blob` | 0; `E_EXIST` one is held; `E_NOENT` this machine cannot open it; `E_INVAL` |
//! | `OP_LOCK_STATE` 35 | `pid u32` | 0 + `set u8` + count + blob (empty when none) |
//!
//! A count is `failures u32 | wait_ms u32`. Each blob is what to keep on
//! disk (`blob.rs`).

use alloc::vec::Vec;

use crate::wipe::secure_zero;

pub const OP_LOCK_SET: u16 = 32;
pub const OP_LOCK_CHECK: u16 = 33;
pub const OP_LOCK_LOAD: u16 = 34;
pub const OP_LOCK_STATE: u16 = 35;
pub const OPS: [u16; 4] = [OP_LOCK_SET, OP_LOCK_CHECK, OP_LOCK_LOAD, OP_LOCK_STATE];

pub const HDR_LEN: usize = 8;
pub const REPLY_HDR_LEN: usize = 8;
pub const COUNT_LEN: usize = 8;

pub const OK: i32 = 0;
/// The passphrase is not the one set.
pub const E_WRONG: i32 = -1;
/// No passphrase is set, or this machine cannot open the one given.
pub const E_NOENT: i32 = -2;
/// Too soon after wrong entries; the count says how long.
pub const E_WAIT: i32 = -11;
pub const E_NOMEM: i32 = -12;
pub const E_ACCES: i32 = -13;
/// A record is held already.
pub const E_EXIST: i32 = -17;
pub const E_INVAL: i32 = -22;

/// Bytes that are wiped when they are let go: every request that carries a
/// passphrase is built in one.
pub struct Secret(Vec<u8>);

impl Secret {
    pub fn as_slice(&self) -> &[u8] {
        &self.0
    }
}

impl Drop for Secret {
    fn drop(&mut self) {
        secure_zero(&mut self.0);
    }
}

fn header(seq: u32, op: u16, body_len: usize) -> Secret {
    let mut v = Vec::with_capacity(HDR_LEN + body_len);
    v.extend_from_slice(&seq.to_le_bytes());
    v.extend_from_slice(&op.to_le_bytes());
    v.extend_from_slice(&0u16.to_le_bytes());
    Secret(v)
}

fn len16(b: &[u8]) -> Option<[u8; 2]> {
    u16::try_from(b.len()).ok().map(u16::to_le_bytes)
}

/// `OP_LOCK_SET`: `current` empty when none is set.
pub fn set_request(seq: u32, pid: u32, current: &[u8], new: &[u8]) -> Option<Secret> {
    let (cl, nl) = (len16(current)?, len16(new)?);
    let mut s = header(seq, OP_LOCK_SET, 8 + current.len() + new.len());
    s.0.extend_from_slice(&pid.to_le_bytes());
    s.0.extend_from_slice(&cl);
    s.0.extend_from_slice(&nl);
    s.0.extend_from_slice(current);
    s.0.extend_from_slice(new);
    Some(s)
}

pub fn check_request(seq: u32, pid: u32, passphrase: &[u8]) -> Option<Secret> {
    let l = len16(passphrase)?;
    let mut s = header(seq, OP_LOCK_CHECK, 6 + passphrase.len());
    s.0.extend_from_slice(&pid.to_le_bytes());
    s.0.extend_from_slice(&l);
    s.0.extend_from_slice(passphrase);
    Some(s)
}

pub fn load_request(seq: u32, pid: u32, blob: &[u8]) -> Secret {
    let mut s = header(seq, OP_LOCK_LOAD, 4 + blob.len());
    s.0.extend_from_slice(&pid.to_le_bytes());
    s.0.extend_from_slice(blob);
    s
}

pub fn state_request(seq: u32, pid: u32) -> Secret {
    let mut s = header(seq, OP_LOCK_STATE, 4);
    s.0.extend_from_slice(&pid.to_le_bytes());
    s
}

fn u32_at(b: &[u8], at: usize) -> Option<u32> {
    let w = b.get(at..at + 4)?;
    Some(u32::from_le_bytes([w[0], w[1], w[2], w[3]]))
}

fn u16_at(b: &[u8], at: usize) -> Option<usize> {
    let w = b.get(at..at + 2)?;
    Some(usize::from(u16::from_le_bytes([w[0], w[1]])))
}

/// `OP_LOCK_SET`'s body: pid, current, new.
pub fn parse_set(body: &[u8]) -> Option<(u32, &[u8], &[u8])> {
    let pid = u32_at(body, 0)?;
    let (cl, nl) = (u16_at(body, 4)?, u16_at(body, 6)?);
    if body.len() != 8 + cl + nl {
        return None;
    }
    Some((pid, &body[8..8 + cl], &body[8 + cl..]))
}

/// `OP_LOCK_CHECK`'s body: pid, passphrase.
pub fn parse_check(body: &[u8]) -> Option<(u32, &[u8])> {
    let pid = u32_at(body, 0)?;
    let l = u16_at(body, 4)?;
    (body.len() == 6 + l).then(|| (pid, &body[6..]))
}

/// `OP_LOCK_LOAD`'s body: pid, blob.
pub fn parse_load(body: &[u8]) -> Option<(u32, &[u8])> {
    Some((u32_at(body, 0)?, body.get(4..)?))
}

/// `OP_LOCK_STATE`'s body: pid.
pub fn parse_state(body: &[u8]) -> Option<u32> {
    (body.len() == 4).then(|| u32_at(body, 0)).flatten()
}

/// Wrong entries in a row and the wait before the next attempt.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Count {
    pub failures: u32,
    pub wait_ms: u32,
}

impl Count {
    pub fn encode(&self) -> [u8; COUNT_LEN] {
        let mut out = [0u8; COUNT_LEN];
        out[0..4].copy_from_slice(&self.failures.to_le_bytes());
        out[4..8].copy_from_slice(&self.wait_ms.to_le_bytes());
        out
    }

    pub fn decode(b: &[u8]) -> Option<Count> {
        Some(Count { failures: u32_at(b, 0)?, wait_ms: u32_at(b, 4)? })
    }
}

/// A reply's status and body, or None when it is shorter than a header or
/// answers another request.
pub fn reply(bytes: &[u8], seq: u32) -> Option<(i32, &[u8])> {
    if u32_at(bytes, 0)? != seq {
        return None;
    }
    let status = u32_at(bytes, 4)? as i32;
    Some((status, &bytes[REPLY_HDR_LEN..]))
}

/// What a reply tells about the record.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Answer<'a> {
    /// Taken: the blob to keep (empty for a load).
    Done(&'a [u8]),
    /// The passphrase was wrong: the count, and the blob to keep with it.
    Wrong(Count, &'a [u8]),
    /// Too soon: the count says how long to wait.
    Wait(Count),
    /// No record is set (check), or this machine cannot open it (load).
    NoRecord,
    /// A record is held already (load).
    Held,
    /// Refused: the status.
    Refused(i32),
}

pub fn answer(status: i32, body: &[u8]) -> Answer<'_> {
    match status {
        OK => Answer::Done(body),
        E_WRONG => match Count::decode(body) {
            Some(c) => Answer::Wrong(c, &body[COUNT_LEN..]),
            None => Answer::Refused(E_INVAL),
        },
        E_WAIT => Count::decode(body).map_or(Answer::Refused(E_INVAL), Answer::Wait),
        E_NOENT => Answer::NoRecord,
        E_EXIST => Answer::Held,
        other => Answer::Refused(other),
    }
}

/// `OP_LOCK_STATE`'s answer: whether a record is held, its count, its blob.
pub fn parse_standing(body: &[u8]) -> Option<(bool, Count, &[u8])> {
    let (&set, rest) = body.split_first()?;
    let count = Count::decode(rest)?;
    Some((set == 1, count, &rest[COUNT_LEN..]))
}

/// `OP_LOCK_STATE`'s answer body.
pub fn standing(set: bool, count: Count, blob: &[u8]) -> Vec<u8> {
    let mut v = Vec::with_capacity(1 + COUNT_LEN + blob.len());
    v.push(u8::from(set));
    v.extend_from_slice(&count.encode());
    v.extend_from_slice(blob);
    v
}
