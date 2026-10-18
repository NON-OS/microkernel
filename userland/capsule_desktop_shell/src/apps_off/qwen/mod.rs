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
 * Qwen from the dock or the Launchpad: the conversation in a window of its
 * own, on the tier chosen at setup or in Settings, as `qwen window` opens it
 * from the Terminal. With none chosen, the default, by the rule the
 * Terminal and setup share (`default_tier`: the stick tier when it fits,
 * else the largest that fits), and a toast says it is the default. The
 * kernel queues the run for init, so a click that succeeds has no pid to
 * give and the window appears a moment later.
 */

/* Which tier runs when none is named, and the fit rule it weighs by. */
#[path = "../../../../capsule_model_fetch/src/default_tier.rs"]
mod default_tier;
#[path = "../../../../capsule_model_fetch/src/need.rs"]
mod need;
/* The memory the kernel counts, read as `qwen tiers` reads it. */
#[path = "../../../../capsule_model_fetch/src/tiers/memory.rs"]
mod memory;
mod run;
mod said;

pub use run::{open, SERVICE};
pub use said::{default_said, last, why};
