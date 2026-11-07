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

//! Init's driver stages and their declared needs, included by path as the
//! kernel builds them (src/userspace/init/spawn_plan/order.rs).

#[path = "../../../../src/userspace/init/spawn_plan/late.rs"]
pub mod late;
#[path = "../../../../src/userspace/init/spawn_plan/once.rs"]
pub mod once;
#[path = "../../../../src/userspace/init/spawn_plan/order.rs"]
pub mod order;

#[cfg(test)]
const ORCHESTRATOR: &str =
    include_str!("../../../../src/userspace/init/spawn_plan/orchestrator.rs");

/// Where init spawns `call` (as the orchestrator's match arm names it, such
/// as `super::drivers_nic::spawn();`): its stage's place in `order::ORDER`.
#[cfg(test)]
pub fn rank(call: &str) -> usize {
    let call = call.trim_end_matches(';');
    let arm = ORCHESTRATOR
        .lines()
        .map(str::trim)
        .find(|l| l.ends_with(&format!("=> {call},")))
        .unwrap_or_else(|| panic!("no stage spawns {call}"));
    let stage = arm.split(" =>").next().unwrap_or_default();
    order::ORDER
        .iter()
        .position(|s| format!("{s:?}") == stage)
        .unwrap_or_else(|| panic!("{stage} is not in the order"))
}

#[cfg(test)]
mod tests;
