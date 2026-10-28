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

//! The listings whose home keeps more than declared::HOME, by the listing
//! key their policy carries (home_key.rs): another publisher's listing of
//! the same name has another key and the ordinary home. Self-contained, so
//! market_proofs recomputes each key from its catalogue listing.

/// linux.monero, published by NONOS. Its wallet keeps a 32-byte hash for
/// each block it has scanned, about 21 MB for a wallet made today, and on
/// every save writes that cache whole beside the last before it replaces
/// it: two copies at once, past 16 MiB each.
pub const MONERO: [u8; 32] = [
    0x90, 0xd6, 0x99, 0x72, 0x00, 0x75, 0x3e, 0xa7, 0xf8, 0x14, 0xa4, 0x9f, 0x79, 0x2a, 0x4d, 0x34,
    0xf3, 0xd0, 0xf1, 0x90, 0x35, 0x57, 0x04, 0xc0, 0xf3, 0x25, 0x88, 0x22, 0x76, 0x00, 0x5d, 0x4e,
];

/// What its home keeps: room for both copies and the chain's growth.
pub const MONERO_HOME: u64 = 64 << 20;

/// Each listing that keeps more, and how much.
pub const LARGER: &[([u8; 32], u64)] = &[(MONERO, MONERO_HOME)];
