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

use nonos_market_proto::ReplyError;

#[path = "failure_text.rs"]
mod failure_text;

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
