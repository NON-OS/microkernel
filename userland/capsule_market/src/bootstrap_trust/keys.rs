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

//! The operators whose catalogues this machine will read.

/// Marketplace operator, v1.
pub(super) const NOX_OPERATOR_V1: [u8; 32] = [
    0xa7, 0xc9, 0x2d, 0xb2, 0x4d, 0x99, 0xe7, 0xba, 0xee, 0x8b, 0x45, 0xa0, 0x6d, 0xc3, 0x53, 0xcc,
    0xd4, 0x14, 0x26, 0x22, 0xc1, 0xa9, 0x0a, 0x52, 0xb5, 0x32, 0x7d, 0xb2, 0xe6, 0xd1, 0x78, 0x11,
];

// One entry, deliberately.
pub(super) const TRUSTED_OPERATORS: &[[u8; 32]] = &[NOX_OPERATOR_V1];
