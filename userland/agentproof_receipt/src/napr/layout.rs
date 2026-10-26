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

//! The format's constants: its head, each field's bounds, the challenge's domain.

pub const MAGIC: &[u8; 4] = b"NAPR";
pub const VERSION: u16 = 0;
pub const HEAD: usize = 6;
pub const RECORD: usize = 240;
pub const BOX_MAX: usize = 255;
pub const POLICY: usize = 256;
pub const LAUNCHER: usize = 32;
/// nsh's cap on a version 5 document (docs/design/nsh-v1-wire.md).
pub const DOC_MAX: usize = 1088 * 1024;
/// One MkAttestEntries entry (src/security/attest_registry/format.rs).
pub const ENTRY: usize = 45;
pub const ENTRIES_MAX: usize = 4096 * ENTRY;
/// One PCR 14 event's body: certificate digest, verifier digest, kind.
pub const CAPSULE: usize = 65;
pub const CAPSULES_MAX: usize = 4096 * CAPSULE;

/// The challenge the doc answers: BLAKE3 over this domain and every byte of
/// the receipt before the doc's length.
pub const CHALLENGE_DOMAIN: &[u8] = b"NONOS-AGENTPROOF-RECEIPT-v0";
