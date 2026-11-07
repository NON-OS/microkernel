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
//! What a job does once its last frame is on the wire: what the old sender
//! did right after its writes, now run when the clock lets them out.

extern crate alloc;

use alloc::vec::Vec;

use crate::ack::ledger::{Resend, LEDGER_RECIPIENT};
use crate::mixnet::Encoded;
use crate::state::TABLE;

pub enum Done {
    /// Nothing to record: a frame of the older format.
    Nothing,
    /// A client's message: its reply blocks are now the far end's, and its
    /// fragments wait on acknowledgements.
    Message {
        pid: u32,
        id: u32,
        recipient: [u8; LEDGER_RECIPIENT],
        fragments: Vec<Encoded>,
        surbs: u32,
    },
    /// A top up of reply blocks, held like any message.
    TopUp { recipient: [u8; LEDGER_RECIPIENT], fragments: Vec<Encoded> },
    /// One fragment sent again.
    Resent { recipient: [u8; LEDGER_RECIPIENT], resend: Resend, wait_ms: i64 },
}

pub fn finish(done: Done) {
    match done {
        Done::Nothing => {}
        Done::Message { pid, id, recipient, fragments, surbs } => {
            TABLE.lock().with_mut(pid, id, |s| s.surbs.sent(surbs));
            crate::server::record(&recipient, fragments);
        }
        Done::TopUp { recipient, fragments } => crate::server::record(&recipient, fragments),
        Done::Resent { recipient, resend, wait_ms } => {
            crate::server::resent(&recipient, resend, wait_ms);
        }
    }
}
