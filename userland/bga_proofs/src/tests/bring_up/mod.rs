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

//! One bring-up attempt end to end against `libc_shim`'s broker and kernel:
//! the claim and PCI command, both BAR mappings, the DISPI ID, the firmware's
//! state read and written back, video memory zeroed, the mode kept or set
//! within video memory and the mapping and read back, and the handles'
//! give-back.

pub mod adapter;
mod attempt;
mod clamp;
mod left;

mod check_tests;
mod firmware_tests;
mod kept_tests;
mod not_taken_tests;
mod restore_tests;
mod setup_bound_tests;
mod setup_claim_tests;
mod setup_confirm_tests;
mod setup_tests;
mod video_tests;
mod write_back_tests;
