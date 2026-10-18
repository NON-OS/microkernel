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

//! Ranges firmware keeps for itself (the motherboard resource devices'
//! _CRS on ACPI machines), and the broker's refusal to hand any of them to
//! a driver. Arch-neutral: with nothing published, nothing is refused.
//! `wire` and `judge` are pure; `kernel_proofs` mounts both.

mod judge;
mod say;
mod store;
pub mod wire;

pub use judge::{conflict, covered};
pub use say::{refused, refuses};
pub use store::{add, conflict_with, count, kept_cover};
pub use wire::{decode as decode_ranges, Reservation, IO, MAX_RANGES, MEMORY, RECORD_LEN};
