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

//! The prover's and the proof fetch's refusals by name (I-B9, the plan): a
//! sentence for each, OUTSIDE said plainly and apart from PROVER. The codes
//! are the plan's; A's nonos_proof_abi::wire carries the same numbers.

pub const STALE_REGISTRY: i32 = -1101;
pub const POLICY: i32 = -1102;
pub const NOT_ENROLLED: i32 = -1103;
pub const PROVER: i32 = -1104;
pub const OUTSIDE: i32 = -1105;

pub const NO_RELAY: i32 = -1201;
pub const PENDING: i32 = -1202;
pub const ROUTE: i32 = -1203;
pub const ANCHOR: i32 = -1204;
pub const NOT_CONTAINER: i32 = -1205;
pub const RELAY: i32 = -1206;

/// Why the proof was not made; None for a status the prover does not name.
pub fn prove_refused(status: i32) -> Option<&'static [u8]> {
    Some(match status {
        STALE_REGISTRY => b"The device registry here is out of date: place the owner's newest registry transcript, then prove again",
        POLICY => b"Its guest policy is not one this machine can prove a run against",
        NOT_ENROLLED => b"This machine is not in the device registry, so it cannot prove",
        PROVER => b"The prover failed, so nothing was proved; try again",
        OUTSIDE => b"A call outside the set succeeded: nothing true to prove",
        _ => return None,
    })
}

/// Where the proof's upload or download stands; None for another status.
pub fn fetch_said(status: i32) -> Option<&'static [u8]> {
    Some(match status {
        NO_RELAY => b"No proof relay is set up yet; the proof stays on this machine",
        PENDING => b"The proof is still downloading",
        ROUTE => b"No private route, Nym or Anyone, reaches the proof relay now",
        ANCHOR => b"The relay's bytes are not the proof asked for",
        NOT_CONTAINER => b"What the relay holds is not a proof container",
        RELAY => b"The proof relay did not answer as it should; try again later",
        _ => return None,
    })
}
