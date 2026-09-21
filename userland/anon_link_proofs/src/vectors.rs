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

//! The captured CERTS cell and TLS leaf certificate.

/// A CERTS cell from the relay named in vectors/source.txt, byte for byte.
pub const CERTS: &[u8] = include_bytes!("../vectors/certs_cell.bin");

/// The DER of the certificate that TLS session ran under.
pub const LEAF: &[u8] = include_bytes!("../vectors/tls_leaf.der");

/// That relay's Ed25519 identity, as its microdescriptor publishes it.
pub const IDENTITY: [u8; 32] = [
    0xa3, 0xfe, 0x69, 0x33, 0x8d, 0xe7, 0x79, 0x0b, 0xd6, 0xb0, 0x8f, 0x31, 0x8d, 0x31, 0xa9, 0x8e,
    0xc8, 0xc1, 0x02, 0x55, 0xab, 0x5c, 0x73, 0x46, 0x48, 0x70, 0x0d, 0xbe, 0xe8, 0x8d, 0xb1, 0x3b,
];

/// Inside both certificates' validity when they were captured.
pub const CAPTURED_AT: u64 = 1_789_800_000;
