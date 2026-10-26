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

//! The question the pane asks, and the line it shows while a proof is
//! made (I-B9a, section 2). Why one did not come is proof_said.rs's.
//!
//! Pure; market_proofs holds it.

use alloc::vec::Vec;

use super::proof_seq::{Phase, DEVICE, ISOLATION, READING};

pub const CHECKED: &[u8] =
    b"Checked on this machine against the box it was sold with. Nothing leaves this machine.";
pub const PROVE_IT: &[u8] = b"Prove it";
pub const NOT_NOW: &[u8] = b"Not now";
pub const READY: &[u8] = b"Proof ready: held here, not sent yet";

/// "Prove how <name> ran?"
pub fn question(name: &[u8]) -> Vec<u8> {
    [&b"Prove how "[..], name, b" ran?"].concat()
}

/// The line while it is made, None once it has ended.
pub fn working(phase: Phase) -> Option<&'static [u8]> {
    Some(match phase {
        Phase::Proving { stage: READING, .. } => b"Reading the sealed run",
        Phase::Proving { stage: DEVICE, .. } => b"Proving this device",
        Phase::Proving { stage: ISOLATION, .. } | Phase::Fetching { .. } => b"Proving the box held",
        Phase::Held | Phase::Failed(..) => return None,
        _ => b"Starting the prover",
    })
}
