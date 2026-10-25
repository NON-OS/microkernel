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

//! PCR 14's events as the kernel writes them: one per admitted capsule,
//! 65 bytes of certificate digest, verifier digest and kind, the event's
//! SHA-256 its body's.

use super::tcg_log::Log;

pub const EV_NONOS_CAPSULE: u32 = 0x4E4F_4E01;
pub const PROOF_VERIFIED: u8 = 1;
pub const PATH_ONLY: u8 = 2;
pub const DEVELOPER: u8 = 3;
pub const LOCAL_BUILD: u8 = 4;

/// The verifier digest a path-only capsule names, and no other may.
pub fn path_only_tag() -> [u8; 32] {
    super::tcg_event::digest(b"nonos capsule path-only")
}

pub fn capsule_body(cert: [u8; 32], verifier: [u8; 32], kind: u8) -> Vec<u8> {
    [&cert[..], &verifier[..], &[kind]].concat()
}

impl Log {
    pub fn capsule(&mut self, cert: [u8; 32], verifier: [u8; 32], kind: u8) -> &mut Self {
        self.note(14, EV_NONOS_CAPSULE, &capsule_body(cert, verifier, kind))
    }

    /// The three proof-verified capsules a default host admitted.
    pub fn with_capsules(mut self) -> Self {
        for i in 1..=3u8 {
            self.capsule([i; 32], [0xC0 + i; 32], PROOF_VERIFIED);
        }
        self
    }
}
