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

//! The session record, `nsh-record/1`, as the host writes it: the header
//! at `AUTH_OK`, one entry per event, the chain over them, a checkpoint
//! every so many entries, and the footer the host keys sign
//! (docs/nsh/nsh-v1-record.md). The record is held to the host's cap with
//! room kept for the entries every session ends with and the footer.

mod bytes;
mod ckpt;
mod entry;
mod footer;
mod header;
mod kind;
mod output;
mod start;

pub(crate) use ckpt::ckpt_digest;
pub use ckpt::CKPT_MAGIC;
pub use header::HEADER_MAX;
pub(crate) use header::Header;
pub(crate) use kind::{Kind as EntryKind, Unfit};

use alloc::vec::Vec;
use nonos_hash::Sha512;
use nsh_wire::SESSION_ID_LEN;
use output::Pending;

/// Room kept for the closing entries (output, escalation end, exit) and
/// the footer, so a record at its cap can still be ended and signed.
const CLOSING_ROOM: usize = 3 * (4 + 8 + 1 + 2 + 200) + nsh_wire::record::FOOTER_LEN;

pub(crate) struct Record {
    pub(crate) session_id: [u8; SESSION_ID_LEN],
    bytes: Vec<u8>,
    chain: [u8; 64],
    count: u32,
    last_ms: u64,
    cap: usize,
    ckpt_every: u32,
    output: Pending,
    /// How far into `bytes` the host has been handed.
    handed: usize,
}

pub(crate) fn link(parts: &[&[u8]]) -> [u8; 64] {
    let mut h = Sha512::new();
    parts.iter().for_each(|p| h.update(p));
    h.finalize()
}
