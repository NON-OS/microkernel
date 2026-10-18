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

//! The access point elements the cipher proofs offer: a network that runs
//! GCMP-256 beside CCMP-128, and one that runs GCMP-256 alone.

/// Group CCMP-128; pairwise GCMP-256 then CCMP-128; PSK.
pub const MIXED: [u8; 26] = [
    0x30, 24, 1, 0, 0, 0x0f, 0xac, 4, 2, 0, 0, 0x0f, 0xac, 9, 0, 0x0f, 0xac, 4, 1, 0, 0, 0x0f,
    0xac, 2, 0, 0,
];
/// Group and pairwise GCMP-256 only; PSK.
pub const GCMP_ONLY: [u8; 22] =
    [0x30, 20, 1, 0, 0, 0x0f, 0xac, 9, 1, 0, 0, 0x0f, 0xac, 9, 1, 0, 0, 0x0f, 0xac, 2, 0, 0];

/// The group key a GCMP-256 group cipher is handed.
pub const GTK256: [u8; 32] = [0x33; 32];
