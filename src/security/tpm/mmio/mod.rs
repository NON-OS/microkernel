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

//! The TPM register window, shared by both interfaces.
//!
//! CRB and FIFO parts decode the same 0xFED40000 range, one locality per
//! 4 KiB page, so there is one uncached mapping of it and each driver reads
//! its own register file through it. Mapping the range twice would create
//! two aliases of the same device memory for no gain.

mod access;
mod map;

pub(in crate::security::tpm) use access::{read32, read8, write32, write8};
pub(in crate::security::tpm) use map::{init_window, map_region, TPM_MMIO_BASE};
