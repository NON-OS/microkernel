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

//! What negotiation settled: the AKM, management frame protection and the
//! pairwise and group data ciphers.

use super::*;

/// How the station will protect management frames.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Pmf {
    /// Not negotiated.
    Off,
    /// Negotiated (MFPC set); the access point does not insist on it.
    Capable,
    /// Negotiated and required (MFPC and MFPR set).
    Required,
}

/// The security the join will run.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Selection {
    pub akm: Akm,
    pub pmf: Pmf,
    /// For SAE: derive the password element by hash-to-element (the access
    /// point's RSNXE advertises it) rather than hunting and pecking.
    pub sae_h2e: bool,
    pub pairwise: Cipher,
    pub group: Cipher,
}
