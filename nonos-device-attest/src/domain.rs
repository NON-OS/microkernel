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

//! Domain words, one per hash role, so no value of one role is ever read as
//! another. The leaf domain and the kinds are `nonos-attest-path`'s, shared with
//! the gates; the device and tag domains are this statement's.

/// "NONOSLV3": the leaf domain the gates and the enroll tool use.
pub const LEAF_DOMAIN: u64 = 0x4E4F_4E4F_534C_5633;

/*
 * Frozen with the STARK lane on 2026-09-30, pinned by known answers in
 * tests/kat_tests.rs. Changing either moves every commitment and every tag.
 */
/// "NONOSDV1": a device commitment.
pub const DEVICE_DOMAIN: u64 = 0x4E4F_4E4F_5344_5631;
/// "NONOSTG1": a device tag.
pub const TAG_DOMAIN: u64 = 0x4E4F_4E4F_5354_4731;

/// The kinds the gates use, as field words.
pub const KIND_KERNEL: u64 = nonos_attest_path::Kind::Kernel as u64;
pub const KIND_BOOTLOADER: u64 = nonos_attest_path::Kind::Bootloader as u64;

/// The scope derivation: a verifier names itself and a window, never the device.
pub const SCOPE_DOMAIN: &[u8] = b"NONOS-ATTEST-SCOPE-v1";
