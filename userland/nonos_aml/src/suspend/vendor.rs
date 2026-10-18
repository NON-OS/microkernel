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

//! The Low Power S0 Idle _DSM's two function sets: Microsoft's and Intel's;
//! and AMD's UUID, only looked for (`s0_seen`).
//! Both UUIDs and their function numbers are recalled from the vendors'
//! published documents (Microsoft's "Modern Standby firmware
//! notifications", Intel's Low Power S0 Idle _DSM), which could not be
//! read here: unverified.

/// 11E00D56-CE64-47CE-837B-1F898F9AA461, in ToUUID's byte order.
pub const MICROSOFT_UUID: [u8; 16] = [
    0x56, 0x0D, 0xE0, 0x11, 0x64, 0xCE, 0xCE, 0x47, 0x83, 0x7B, 0x1F, 0x89, 0x8F, 0x9A, 0xA4, 0x61,
];
/// C4EB40A0-6CD2-11E2-BCFD-0800200C9A66, in ToUUID's byte order.
pub const INTEL_UUID: [u8; 16] = [
    0xA0, 0x40, 0xEB, 0xC4, 0xD2, 0x6C, 0xE2, 0x11, 0xBC, 0xFD, 0x08, 0x00, 0x20, 0x0C, 0x9A, 0x66,
];

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Vendor {
    Microsoft,
    Intel,
}

impl Vendor {
    pub fn uuid(self) -> [u8; 16] {
        match self {
            Vendor::Microsoft => MICROSOFT_UUID,
            Vendor::Intel => INTEL_UUID,
        }
    }
}

/// AMD's, recalled the same way: E3F32452-FEBC-43CE-9039-932122D37721, in ToUUID's byte order.
pub const AMD_UUID: [u8; 16] = [
    0x52, 0x24, 0xF3, 0xE3, 0xBC, 0xFE, 0xCE, 0x43, 0x90, 0x39, 0x93, 0x21, 0x22, 0xD3, 0x77, 0x21,
];

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Uuids {
    pub microsoft: bool,
    pub intel: bool,
    pub amd: bool,
}

/// The known UUIDs `body` holds, found in its bytes, never by a call.
pub fn uuids(body: &[u8]) -> Uuids {
    let has = |u: &[u8; 16]| body.windows(16).any(|w| w == u);
    Uuids { microsoft: has(&MICROSOFT_UUID), intel: has(&INTEL_UUID), amd: has(&AMD_UUID) }
}
