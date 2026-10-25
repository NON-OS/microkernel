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

//! What the station keeps of an access point's RSN element (IEEE Std
//! 802.11-2020, 9.4.2.24) to select from.

use super::suite::{CAP_MFPC, CAP_MFPR};

/// The parts of an access point's RSN element the station selects from.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Rsne {
    /// The group data cipher suite.
    pub group: u32,
    /// Whether CCMP-128 is among the pairwise cipher suites.
    pub pairwise_ccmp: bool,
    /// Whether GCMP-256 is among them.
    pub pairwise_gcmp256: bool,
    pub akm_psk: bool,
    pub akm_psk_sha256: bool,
    pub akm_sae: bool,
    /// The RSN Capabilities field.
    pub caps: u16,
    /// The group management cipher suite, when the element names one.
    pub group_mgmt: Option<u32>,
}

impl Rsne {
    /// Management frame protection capable.
    pub fn mfpc(&self) -> bool {
        self.caps & CAP_MFPC != 0
    }

    /// Management frame protection required.
    pub fn mfpr(&self) -> bool {
        self.caps & CAP_MFPR != 0
    }
}
