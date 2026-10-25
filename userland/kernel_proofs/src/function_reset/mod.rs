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

#[cfg(test)]
mod busy_tests;
#[cfg(test)]
mod fake;
pub mod gate;
#[cfg(test)]
mod gate_tests;
pub mod leaving;
#[path = "../../../../src/hardware/broker/claim/reset_rule.rs"]
pub mod reset_rule;
#[path = "../../../../src/hardware/broker/claim/reset_run.rs"]
pub mod reset_run;
#[path = "../../../../src/hardware/broker/claim/reset_wait.rs"]
pub mod reset_wait;
#[cfg(test)]
mod record_tests;
pub mod retire;
#[cfg(test)]
mod retire_fake;
#[cfg(test)]
mod retire_holder;
#[cfg(test)]
mod retire_tests;
#[cfg(test)]
mod tests;
pub mod state;
pub mod still_held;
// Mounted in claim_stage too; the lifecycle fields go unread in this copy.
#[allow(clippy::duplicate_mod, dead_code)]
#[path = "../../../../src/hardware/broker/claim/types.rs"]
pub mod types;
#[cfg(test)]
mod wait_tests;
