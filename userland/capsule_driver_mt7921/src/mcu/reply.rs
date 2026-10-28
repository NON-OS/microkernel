/*
 * NONOS Operating System
 * Copyright (C) 2026 NONOS Contributors (AGPL-3.0-or-later)
 */

//! A command's reply and what it says, as Linux v6.12
//! `mt7921_mcu_parse_response` reads it: the semaphore and patch-finish
//! commands answer in the first byte past the header's first 32; the
//! unified device, BSS and station commands in a `cid` and a status word,
//! and a reply naming another `cid` is not this command's.

use alloc::vec::Vec;

use super::cmd::Cmd;
use super::event::{Event, BODY};
use super::McuError;

/// A reply, kept whole: the RXD, the header, the body.
pub struct Reply {
    pub event: Vec<u8>,
}

impl Reply {
    /// The bytes past the header.
    pub fn body(&self) -> &[u8] {
        self.event.get(BODY..).unwrap_or(&[])
    }

    /// The byte `PATCH_SEM_CONTROL` and `PATCH_FINISH_REQ` answer with.
    pub fn status_byte(&self) -> u8 {
        self.event.get(32).copied().unwrap_or(u8::MAX)
    }
}

/// `None` when `ev` answers some other command (keep waiting); else this
/// command's outcome.
pub fn outcome(cmd: Cmd, ev: &Event) -> Option<Result<Reply, McuError>> {
    let reply = || Reply { event: ev.0.to_vec() };
    if !cmd.uni_status() {
        return Some(Ok(reply()));
    }
    let b = ev.body();
    if b.len() < 8 || b[0] != cmd.id {
        return None;
    }
    let status = u32::from_le_bytes([b[4], b[5], b[6], b[7]]);
    Some(if status == 0 { Ok(reply()) } else { Err(McuError::Status(status)) })
}

/// The replies a caller waits on: those with `seq`, sorted by `outcome`.
pub fn matches(seq: u8, cmd: Cmd, ev: &Event) -> Option<Result<Reply, McuError>> {
    if ev.unsolicited() || ev.seq() != seq {
        return None;
    }
    outcome(cmd, ev)
}
