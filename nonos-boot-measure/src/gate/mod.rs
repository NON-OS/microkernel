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

//! The kernel's decision about the bootloader, whole: what to check, in which
//! order, and what each refusal is. The kernel brings only the TPM's answers
//! and the release key; everything else is here, where it is tested.

mod error;
mod membership;
mod verdict;

pub use error::BootError;
pub use membership::{membership, BOOT_EPOCH, DEPTH};
pub use verdict::{measured, self_reported, Admitted};
