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
#[path = "../../../../src/hardware/broker/pio/access/gate.rs"]
pub mod gate;
#[cfg(test)]
mod gate_tests;
#[path = "../../../../src/hardware/broker/claim/leaving.rs"]
pub mod leaving;
#[path = "../../../../src/hardware/broker/claim/reset_rule.rs"]
pub mod reset_rule;
#[path = "../../../../src/hardware/broker/claim/reset_run.rs"]
pub mod reset_run;
#[path = "../../../../src/hardware/broker/claim/reset_wait.rs"]
pub mod reset_wait;
#[cfg(test)]
mod record_tests;
#[path = "../../../../src/hardware/broker/claim/retire.rs"]
pub mod retire;
#[cfg(test)]
mod retire_fake;
#[cfg(test)]
mod retire_holder;
#[cfg(test)]
mod retire_tests;
#[cfg(test)]
mod tests;
#[path = "../../../../src/hardware/broker/claim/state.rs"]
pub mod state;
#[path = "../../../../src/hardware/broker/claim/still_held.rs"]
pub mod still_held;
#[path = "../../../../src/hardware/broker/claim/types.rs"]
pub mod types;
#[cfg(test)]
mod wait_tests;
