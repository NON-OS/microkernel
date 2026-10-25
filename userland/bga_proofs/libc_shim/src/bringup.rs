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

//! The libc's own bring-up and receive policy, so the serve loop's receive
//! is read by the `recv_ready` the capsule links, not by a copy of it.

#[path = "../../../libc/src/bringup/deadline.rs"]
pub mod deadline;
#[path = "../../../libc/src/bringup/policy.rs"]
pub mod policy;
#[path = "../../../libc/src/bringup/recv_turn.rs"]
pub mod recv_turn;
#[path = "../../../libc/src/bringup/run.rs"]
pub mod run;
#[path = "../../../libc/src/bringup/say.rs"]
pub mod say;
#[path = "../../../libc/src/bringup/schedule.rs"]
pub mod schedule;
