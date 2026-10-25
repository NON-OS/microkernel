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

//! The topic0 of NoxListingRegistry's two events.

pub const PUBLISHED: &str = "Published(bytes32,bytes32,bytes32,bytes32,string)";
/// keccak256(PUBLISHED), pinned by a test.
pub const PUBLISHED_TOPIC: [u8; 32] = [
    0x13, 0xf2, 0x63, 0xf2, 0x46, 0x2f, 0x17, 0xe9, 0x67, 0x1a, 0xff, 0x07, 0x8e, 0x49, 0x3f, 0x25,
    0x97, 0x0f, 0x61, 0xab, 0xb0, 0xc3, 0xd2, 0xda, 0xe4, 0xc1, 0x1a, 0xf1, 0x69, 0x62, 0x6d, 0x74,
];
pub const WITHDRAWN: &str = "Withdrawn(bytes32,bytes32)";
/// keccak256(WITHDRAWN), pinned by a test.
pub const WITHDRAWN_TOPIC: [u8; 32] = [
    0x62, 0x38, 0x8c, 0x59, 0xa1, 0xde, 0xee, 0xf5, 0xc4, 0x3b, 0xd1, 0xe4, 0x53, 0xc0, 0x9d, 0x8f,
    0xd2, 0x55, 0xc3, 0x1f, 0x65, 0x17, 0x00, 0xe1, 0xf6, 0x06, 0x33, 0x99, 0x58, 0xb8, 0x8b, 0x1c,
];
