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

//! A throwaway operator key, fixed so the trust list can hold it as a
//! constant, the way the dev tree's capsule holds the key made in that tree.
//! Nothing outside these tests signs with it.

pub const DEV_SEED: [u8; 32] = [0xd5; 32];

/// The ed25519 public key of DEV_SEED. Were it wrong, the CLI would refuse to
/// sign under it and every test here would fail.
pub const DEV_OPERATOR: [u8; 32] = [
    0xa1, 0x18, 0x04, 0x33, 0x59, 0xe7, 0xcb, 0x45, 0xea, 0x8b, 0xb2, 0x01, 0xe7, 0xc8, 0xb9, 0x43,
    0xf1, 0x34, 0x56, 0xc0, 0xa4, 0xd1, 0xb1, 0x1d, 0x43, 0x43, 0xa7, 0x82, 0x80, 0x89, 0xfb, 0xba,
];

pub(super) const TRUSTED_OPERATORS: &[[u8; 32]] = &[DEV_OPERATOR];
