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

/*
 * The Qwen step: the pinned tiers this machine can run, by fit to its memory.
 */

mod default;
/* The tier that runs when none is chosen, as the dock and the Terminal pick it. */
#[path = "../../../capsule_model_fetch/src/default_tier.rs"]
mod default_tier;
mod labels;
mod memory;
/* What a tier needs in memory, the model fetcher's own rule. */
#[path = "../../../capsule_model_fetch/src/need.rs"]
mod need;
mod pins;
mod state;
/* Whether this is QEMU's software CPU, read as the Linux personality reads it. */
#[path = "../../../capsule_linux/src/linux/install/tcg.rs"]
mod tcg;

pub use labels::label;
pub use default::For;
pub use need::{memory as need_of, STICK_TIER};
pub use state::QwenState;
