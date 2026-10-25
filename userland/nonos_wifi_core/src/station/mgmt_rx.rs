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

//! What the link makes of a management frame, and the reason codes it reads.

use alloc::vec::Vec;

use crate::dot11::auth::{SUBTYPE_DEAUTH, SUBTYPE_DISASSOC};

/// What a received management frame calls for.
#[derive(Debug, PartialEq, Eq)]
pub enum MgmtRx {
    /// The AP ended the association with this reason; the station is down.
    Left(u16),
    /// Send this to the AP: an SA Query response, protected as `tx_frame`
    /// protects (here under a software key, by the radio otherwise).
    Reply(Vec<u8>),
    /// Start an SA Query (`sa_query_start`) to learn whether the AP still
    /// holds the association.
    SaQueryNeeded,
    /// The AP answered the station's SA Query: the association stands.
    Confirmed,
    Dropped(MgmtDrop),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MgmtDrop {
    NotAssociated,
    Malformed,
    /// Not from the BSS, or addressed to another station.
    NotOurs,
    Unprotected,
    /// No key for it, or its MIC failed.
    Undecryptable,
    Replay,
    /// A management frame the link has nothing to do with.
    NotHandled,
}

pub(super) const REASON_CLASS2: u16 = 6;
pub(super) const REASON_CLASS3: u16 = 7;

/// The reason code of a deauthentication or disassociation body.
pub(super) fn leave_reason(subtype: u8, body: &[u8]) -> Option<u16> {
    let leave = subtype == SUBTYPE_DEAUTH || subtype == SUBTYPE_DISASSOC;
    leave.then(|| body.get(..2).map(|r| u16::from_le_bytes([r[0], r[1]]))).flatten()
}

/// What a tick of the station's SA Query calls for.
#[derive(Debug, PartialEq, Eq)]
pub enum SaTick {
    /// No query running, or none due.
    Idle,
    /// Send this request to the AP.
    Send(Vec<u8>),
    /// The AP never answered: the station has left the association.
    Ended,
}
