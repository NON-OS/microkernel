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

//! Host proofs for the compositor. The `#[path]` includes pull in the real
//! production source so the tests pin the shipping behaviour, not a copy;
//! each group's `mod.rs` holds its includes, and `tests/` the proofs.
//! The state modules are re-exported at the root, where the proofs name them.

extern crate alloc;

pub mod display;
pub mod frame_pacer;
pub mod state;

pub use frame_pacer::{drain_batch, drain_damage};
pub use state::{
    attach, damage, damage_bound, indicator, raise_rule, scene, scene_raise, scene_remove,
    scene_submit, trusted_rule, visible,
};

#[path = "../../compositor/src/sw_blitter/mod.rs"]
pub mod sw_blitter;

// The wire every request arrives on: the header decode the drain runs before
// dispatch, and the encoders a refusal is answered with.
#[path = "../../compositor/src/protocol/mod.rs"]
pub mod protocol;

#[path = "../../compositor/src/requests/mod.rs"]
pub mod requests;

/// Which driver the display log names, over that list.
#[path = "../../compositor/src/setup/discover/order.rs"]
pub mod discover_order;
/// The kernel's display list and hint sources, which the compositor mounts.
#[allow(dead_code)]
#[path = "../../../src/services/registry/displays.rs"]
pub mod displays;

#[path = "../../compositor/src/server/runner/turn_watch.rs"]
pub mod turn_watch;

/// The line a GOP backing the kernel refused is named by (H-06).
#[path = "../../compositor/src/setup/rejected.rs"]
pub mod rejected;

/// The stall watchdog's rule and its lines; the thread and the atomics it
/// reads are the capsule's.
#[path = "."]
pub mod runner {
    #[path = "../../compositor/src/server/runner/stall_line.rs"]
    pub mod stall_line;
    #[path = "../../compositor/src/server/runner/stall_rule.rs"]
    pub mod stall_rule;
    #[path = "../../compositor/src/server/runner/stall_step.rs"]
    pub mod stall_step;
}

#[cfg(test)]
mod k20_tests;
#[cfg(test)]
mod tests;
