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

//! The cryptoblock run never held across a send (L81b part B): the rule
//! that decides when a run must go, a model of a writer asleep or killed in
//! its send beside a later writer and a reader, and the kernel text pinned.

#[path = "../../../../src/fs/cryptoblock/pending_rule.rs"]
mod pending_rule;

#[cfg(test)]
mod glue_drop_tests;
#[cfg(test)]
mod glue_tests;
#[cfg(test)]
mod glue_walk_tests;
#[cfg(test)]
mod program;
#[cfg(test)]
mod rule_tests;
#[cfg(test)]
mod search;
#[cfg(test)]
mod search_tests;
#[cfg(test)]
mod world;
#[cfg(test)]
mod world_leave;
#[cfg(test)]
mod world_ops;
#[cfg(test)]
mod world_send;
#[cfg(test)]
mod world_step;
