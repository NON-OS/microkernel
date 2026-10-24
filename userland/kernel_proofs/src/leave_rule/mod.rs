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

//! The leave phase's schedule (src/hardware/block_device/leave/rule.rs):
//! every leg polled each round, each ended by its own budget or the total,
//! whichever comes first, and a clock that cannot be read ends them all;
//! and one leg's round (leg_poll.rs) on the stand-in transport.

// The leave lines read `name` and `budget_ms` (leave/run.rs, not mounted).
#[allow(dead_code)]
#[path = "../../../../src/hardware/block_device/leave/leg.rs"]
mod leg;
#[path = "../../../../src/hardware/block_device/leave/leg_poll.rs"]
mod leg_poll;
#[path = "../../../../src/hardware/block_device/leave/poll.rs"]
mod poll;
#[path = "../../../../src/hardware/block_device/leave/rule.rs"]
mod rule;

#[cfg(test)]
mod test_clock;

#[cfg(test)]
mod tests;

#[cfg(test)]
mod together_tests;

#[cfg(test)]
mod order_tests;

#[cfg(test)]
mod gone_tests;

#[cfg(test)]
mod leg_end_tests;

#[cfg(test)]
mod leg_rig;

#[cfg(test)]
mod leg_tests;

#[cfg(test)]
mod critical_tests;
