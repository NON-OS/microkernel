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

//! Why a market call brought back no body, and what the store says for it
//! in place of the catalogue.

use alloc::format;
use alloc::vec::Vec;

use nonos_market_proto::status::{E_INVAL, E_MSGSIZE, E_NODATA};
use nonos_market_proto::{status_name, ReplyError};

/// A market call that brought back nothing to read.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Failure {
    /// No reply: no port, the call failed or timed out, or the reply was too
    /// short to carry a status.
    NoReply,
    /// A reply that is not the answer to this call: another service's, or
    /// the market's answer to an earlier call that timed out.
    Mismatch,
    /// A reply whose status was not zero.
    Status(i32),
    /// A zero status and a body that does not parse, or a header naming
    /// more body than arrived.
    Malformed,
}

impl From<ReplyError> for Failure {
    fn from(e: ReplyError) -> Failure {
        match e {
            ReplyError::Short => Failure::NoReply,
            ReplyError::Foreign | ReplyError::Stale => Failure::Mismatch,
            ReplyError::Truncated => Failure::Malformed,
            ReplyError::Status(status) => Failure::Status(status),
        }
    }
}

impl Failure {
    /// What the catalogue pane says, one line per `\n`.
    pub fn catalogue_trouble(self) -> Vec<u8> {
        match self {
            Failure::NoReply => b"market did not answer\npress r to ask again".to_vec(),
            Failure::Mismatch => b"market answered another request\npress r to ask again".to_vec(),
            /*
             * A build without the operator key has no catalogue here, and no
             * signed model catalogue for the fetcher either: only a model
             * already on the disk runs, so that is all this says.
             */
            Failure::Status(E_NODATA) => b"this machine has no signed catalogue (ENODATA)\n\
                so nothing can be installed from here; a Qwen model\n\
                already on this disk still runs with qwen in the Terminal"
                .to_vec(),
            Failure::Status(E_INVAL) => b"the market did not understand the request (EINVAL)\n\
                this window and the market may be from different builds"
                .to_vec(),
            Failure::Status(E_MSGSIZE) => b"the catalogue is too large for one reply (EMSGSIZE)\n\
                so this window cannot show it"
                .to_vec(),
            Failure::Status(status) => match status_name(status) {
                Some(name) => {
                    format!("market refused the catalogue: {name} ({status})\npress r to ask again")
                }
                None => format!(
                    "market refused the catalogue with status {status}\npress r to ask again"
                ),
            }
            .into_bytes(),
            Failure::Malformed => b"market sent a catalogue this store cannot read\n\
                the two may be from different builds; press r to retry"
                .to_vec(),
        }
    }
}
