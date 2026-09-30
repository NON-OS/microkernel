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

//! What first-boot setup keeps when the person chose a mode that keeps state.
//!
//! Two files in the vfs store. The store overwrites a record only with one of
//! the same length, so each version's length is fixed. Setup writes the
//! answers first and the marker last, and the policy service restores the
//! answers only beside a marker, so a half-finished save restores nothing.

mod answers;
mod check;
mod kept;
mod layout;
mod names;
mod older;
mod record;
mod refused;
mod rules;

pub use answers::Answers;
pub use check::{check, check_record};
pub use kept::{Kept, Name, Tier};
pub use layout::{is_done, ANSWERS_LEN, ANSWERS_PATH, ANSWERS_V1_LEN, ANSWERS_V2_LEN};
pub use layout::{DONE, DONE_PATH, SETUP_DIR};
pub use record::Record;
pub use refused::Refused;
pub use rules::{name_ok, tier_ok, NAME_MAX, TIER_MAX};
