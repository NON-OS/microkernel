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

//! How far a join has come, and what one input produced.

use alloc::vec::Vec;

/// How far the association has progressed.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum MlmeState {
    /// Watching beacons for the target SSID.
    Scanning,
    /// A matching BSS was found; authentication (Open System or SAE) sent.
    Authenticating,
    /// Authenticated; association request sent.
    Associating,
    /// Associated; running the four-way handshake.
    FourWay,
    /// Connected: pairwise and group keys installed, link up.
    Connected,
    /// The join failed; `Mlme::failure` says where.
    Failed,
}

/// What one input produced: the frame to transmit next, if any. The link
/// state is read separately from `Mlme::state`.
pub struct MlmeOutput {
    pub tx: Option<Vec<u8>>,
}

impl MlmeOutput {
    pub(in crate::mlme) fn none() -> Self {
        Self { tx: None }
    }

    pub(in crate::mlme) fn send(frame: Vec<u8>) -> Self {
        Self { tx: Some(frame) }
    }
}
