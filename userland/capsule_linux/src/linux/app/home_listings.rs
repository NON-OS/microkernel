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
//! key their policy carries (home_key.rs) or the program it names: an image
//! whose catalogue is signed with its own keys, as a development image's
//! is, keys the same listing differently, and its program is the same.
//! Only how much a home may keep follows; which home is the key's alone.
//! Self-contained, so market_proofs holds each to its catalogue listing.

/// linux.monero, published by NONOS. Its wallet keeps a 32-byte hash for
/// each block it has scanned, about 21 MB for a wallet made today, and on
/// every save writes that cache whole beside the last before it replaces
/// it: two copies at once, past 16 MiB each.
pub const MONERO: [u8; 32] = [
    0x90, 0xd6, 0x99, 0x72, 0x00, 0x75, 0x3e, 0xa7, 0xf8, 0x14, 0xa4, 0x9f, 0x79, 0x2a, 0x4d, 0x34,
    0xf3, 0xd0, 0xf1, 0x90, 0x35, 0x57, 0x04, 0xc0, 0xf3, 0x25, 0x88, 0x22, 0x76, 0x00, 0x5d, 0x4e,
];

/// monero-wallet-cli 0.18.3.3-r1, the program linux.monero's policy names.
pub const MONERO_PROGRAM: [u8; 32] = [
    0x20, 0x73, 0xf1, 0xb6, 0x21, 0xb3, 0xf1, 0x0b, 0xa4, 0x5b, 0x29, 0xa2, 0x7d, 0xe4, 0xe7, 0x83,
    0xb6, 0xd8, 0xd2, 0x49, 0xbd, 0x57, 0x00, 0x88, 0x70, 0x62, 0xc3, 0x87, 0xcc, 0x66, 0x9c, 0x83,
];

/// What its home keeps: room for both copies and the chain's growth.
pub const MONERO_HOME: u64 = 64 << 20;

/// linux.telegram-desktop, published by NONOS. Its first start, before any
/// sign-in, writes an emoji cache of 14.4 MB at scale 1 and 54.4 MB at any
/// larger scale, where it draws its emoji twice the size; in 16 MiB nothing
/// after it, the session or a chat's media, could be written.
pub const TELEGRAM: [u8; 32] = [
    0x32, 0x8f, 0x86, 0xee, 0x03, 0x1a, 0xfc, 0x84, 0x8a, 0x56, 0x42, 0x2e, 0x6b, 0x48, 0x9f, 0x12,
    0x0e, 0xbe, 0xb8, 0x75, 0x92, 0x06, 0x65, 0x38, 0x57, 0xe8, 0x81, 0xde, 0x62, 0x17, 0xae, 0xa9,
];

/// telegram-desktop 5.0.1-r6, the program its catalogue listing's and its
/// app image's policies name.
pub const TELEGRAM_PROGRAM: [u8; 32] = [
    0x80, 0xb6, 0x3d, 0xfe, 0x51, 0x6d, 0x47, 0xa3, 0x0b, 0xce, 0x07, 0x6f, 0x4d, 0x4c, 0xa4, 0xa3,
    0xe5, 0x90, 0x27, 0xe7, 0xdf, 0x51, 0xf4, 0x97, 0xdc, 0x10, 0x01, 0x41, 0xab, 0x02, 0xf2, 0x07,
];

/// What its home keeps: the larger emoji cache and room for the account's
/// own files and media beside it.
pub const TELEGRAM_HOME: u64 = 64 << 20;

/// Each listing that keeps more, by its key or its program, and how much.
pub const LARGER: &[([u8; 32], [u8; 32], u64)] =
    &[(MONERO, MONERO_PROGRAM, MONERO_HOME), (TELEGRAM, TELEGRAM_PROGRAM, TELEGRAM_HOME)];
