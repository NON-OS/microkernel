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

//! ACPI Platform Error Interfaces (ACPI 6.5, chapter 18): where the
//! platform reports hardware errors, and the records it writes, decoded.
//! Pure: the capsule reads the memory, this reads the bytes.

mod bert;
mod block;
mod guids;
mod hest;
mod le;
mod section;
mod source;

pub use bert::bert;
pub use block::{parse_block, Block, Entry, CORRECTED, FATAL};
pub use guids::{MEMORY, PCIE, PROCESSOR};
pub use hest::{hest, ErrorSource};
pub use section::{record, Record};
pub use source::{Ack, Gas, Ghes, POLLED, SCI};
