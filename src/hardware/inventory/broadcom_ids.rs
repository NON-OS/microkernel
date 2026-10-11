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

//! The Broadcom network parts a capsule starts for, the capsule's own ids
//! (kernel_proofs holds them together): the ten 57765-class functions of
//! Linux tg3_pci_tbl, in its order. The 5761, 57780, 5717 class, 5762,
//! 5725 and 5727 are other tg3 families, which no capsule drives yet.
//!
//! Pure, with no imports, so host proofs can include it by path.

pub const TG3_DEVICE_IDS: [u16; 10] =
    [0x16B1, 0x16B5, 0x16B0, 0x16B4, 0x16B2, 0x16B6, 0x1682, 0x1686, 0x16B7, 0x16B3];
