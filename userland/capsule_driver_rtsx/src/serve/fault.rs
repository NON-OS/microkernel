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

//! How a failure on the card is answered, and what it does to the medium.
//! A refusal never reached the card. A card failure did, and every one but
//! the write-protect switch leaves the card to be identified again before
//! it serves another request (C1): a timeout is ended by a power cycle and
//! a fresh identification, which also tells whether it is the same card.

use crate::error::RtsxError;
use crate::protocol::{E_IO, E_NOMEDIUM, E_ROFS, E_TIMEDOUT};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fault {
    /// A request this service refuses, with its status.
    Refused(i32),
    Card(RtsxError),
}

pub const fn errno(e: RtsxError) -> i32 {
    match e {
        RtsxError::WriteProtected => E_ROFS,
        RtsxError::Removed => E_NOMEDIUM,
        RtsxError::CmdTimeout | RtsxError::DataTimeout | RtsxError::CardBusy => E_TIMEDOUT,
        RtsxError::RegisterTimeout | RtsxError::PhyTimeout => E_TIMEDOUT,
        _ => E_IO,
    }
}

impl Fault {
    pub const fn status(self) -> i32 {
        match self {
            Fault::Refused(status) => status,
            Fault::Card(e) => errno(e),
        }
    }

    /// Whether the card must be identified again before it serves another.
    pub const fn unsettles(self) -> bool {
        matches!(self, Fault::Card(e) if !matches!(e, RtsxError::WriteProtected))
    }
}
