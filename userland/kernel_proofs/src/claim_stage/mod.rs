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
 * Where a claim is in its life and what each stage allows (L65, L66), the
 * kernel's rule included by path, and a model of a release racing a grant
 * path: no grant whose check passed outlives the release's drain.
 */

mod exit_steps;
mod exit_world;
mod making_tests;
mod model_exit_tests;
mod model_reclaim_tests;
mod model_tests;
mod over_tests;
mod reclaim_world;
#[path = "../../../../src/hardware/broker/claim/stage_rule.rs"]
pub mod stage_rule;
mod tests;
#[path = "../../../../src/hardware/broker/claim/types.rs"]
pub mod types;
