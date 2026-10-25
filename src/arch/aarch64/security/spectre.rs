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

//! Speculation: one decision per CPU, from its identification registers,
//! its MIDR and the firmware's SMCCC answers (`decide`), then acted on:
//! SSBD at once, the rest on every entry from EL0 through the word the
//! vectors read (`entry_word`).

mod answer;
mod barrier;
mod branch;
mod cores;
mod decide;
mod entry;
mod entry_word;
mod history;
mod ids;
mod init;
mod load;
mod plan;
mod read;
mod report;
mod ssbs;
mod words;

pub use barrier::{clear_prediction_state, speculative_barrier};
pub(crate) use init::boot_exposed;
pub use init::init_spectre_mitigations;
pub use load::speculation_safe_load;
pub use plan::{Bhb, Meltdown, Plan, Ssbd, Why, V2};
