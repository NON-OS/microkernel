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

//! What the record checker found: a complete record, a record cut short
//! at a checkpoint, or the step that refused it.

use super::Mismatch;
use crate::{Half, HostRefusal};
use nsh_wire::WireError;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Verdict {
    /// Every step passed, the footer's signature included.
    Complete {
        entries: u32,
        end_ms: u64,
        /// The footer's end reason, 1 to 6.
        end_reason: u8,
        /// The record holds a `BOX` entry, a kind that is still provisional
        /// (nsh-v1-record.md): no host should sign one yet.
        boxed: bool,
    },
    /// There is no footer. Entries 0 to `seq` are checked and signed by a
    /// checkpoint; nothing after `seq` is.
    CutShort { seq: u32, boxed: bool },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RecordRefusal {
    /// Step 1: the record does not parse (and there is no checkpoint to
    /// check a footerless record against).
    Parse(WireError),
    /// The header names another session, handshake or user than expected.
    Unbound(Mismatch),
    /// Step 2: the document given is not the one the header names.
    DocHash,
    /// Steps 2 to 4: the document, the challenge, the enrolment or the
    /// firmware PCRs.
    Host(HostRefusal),
    /// Step 5: the chain does not recompute to the footer's value.
    Chain,
    /// Step 6: the host keys' signature over the footer.
    Sig(Half),
    Ckpt(CkptRefusal),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CkptRefusal {
    Parse(WireError),
    /// It names another session than the record's.
    Session,
    /// The host keys' signature over it; the half that failed.
    Sig(Half),
    /// It covers an entry the record does not hold.
    Beyond,
    /// The record's chain after its entry is not the value it signs.
    Chain,
}
