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

//! Host-runnable proofs for the bootloader's rollback floor. The real
//! `security::tpm_nv` read and raise sequences and the floor rule are pulled in
//! via `#[path]` and run against a TPM scripted to the specification, so the
//! invariants are proven about the code that gates a kernel boot.

extern crate alloc;

pub mod display;
pub mod handoff;
pub mod image_format;
pub mod menu;
pub mod paging;
pub mod security;

#[cfg(test)]
mod display_mode_tests;
#[cfg(test)]
mod display_order_tests;
#[cfg(test)]
mod fb_window_tests;
#[cfg(test)]
mod floor_cmd_tests;
#[cfg(test)]
mod floor_raise_tests;
#[cfg(test)]
mod floor_rule_tests;
#[cfg(test)]
mod floor_seq_tests;
#[cfg(test)]
mod image_format_tests;

#[cfg(kani)]
mod kani_proofs;
#[cfg(test)]
mod scripted_tpm;
