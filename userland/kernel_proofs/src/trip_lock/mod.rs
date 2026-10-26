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
 * The trip lock (L81b): the kernel's own rows, the rule table over them,
 * and the world model that searches every interleaving of two writers and
 * two readers with kills, a stop, a stale mark, an error path, nested reads
 * and a reused pid, on one CPU and two, with the knob table it must give.
 */

use crate::proofs::proofs_8::transport_takeover::transport_rule;
#[path = "../../../../src/services/lifecycle/trip_rows.rs"]
mod trip_rows;
#[path = "../../../../src/services/lifecycle/trip_turn.rs"]
mod trip_turn;

#[cfg(test)]
mod glue_marks_tests;
#[cfg(test)]
mod glue_tests;
#[cfg(test)]
mod rule_more_tests;
#[cfg(test)]
mod rule_tests;
#[cfg(test)]
mod search;
#[cfg(test)]
mod tests;
#[cfg(test)]
mod world;
#[cfg(test)]
mod world_ask;
#[cfg(test)]
mod world_judge;
#[cfg(test)]
mod world_more;
#[cfg(test)]
mod world_run;
#[cfg(test)]
mod world_state;
#[cfg(test)]
mod world_step;
