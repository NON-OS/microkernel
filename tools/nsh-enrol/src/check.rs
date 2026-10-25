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

//! The client's own checks of a document that need no enrolment or policy:
//! it parses as version 5, answers the challenge if one is given, its quote
//! binds its challenge, registry root and DMA posture and is signed by its
//! AK, and its event log replays to the quoted PCRs 0, 1, 2, 4 and 7.

use nsh_attest::consts::REPLAYED_PCRS;
use nsh_attest::log::{replay, Replayed};
use nsh_attest::steps::quote;
use nsh_attest::{Doc, Refusal};

use crate::Error;

pub fn checked(bytes: &[u8], challenge: Option<[u8; 32]>) -> Result<(Doc<'_>, Replayed), Error> {
    let doc = Doc::parse(bytes).map_err(Error::Doc)?;
    if challenge.is_some_and(|c| c != *doc.challenge) {
        return Err(Error::Challenge);
    }
    quote(&doc).map_err(Error::Doc)?;
    let log = replay(doc.log).map_err(|f| Error::Doc(Refusal::Log(f)))?;
    for i in REPLAYED_PCRS {
        if doc.pcr(i) != log.pcrs.get(usize::from(i)) {
            return Err(Error::Replay(i));
        }
    }
    Ok((doc, log))
}
