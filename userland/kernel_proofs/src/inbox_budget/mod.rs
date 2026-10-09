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
 * The kernel's IPC inbox byte budget, included by path.
 *
 * An inbox counted messages only, so 1024 messages of up to 1 MiB each could
 * be queued in one inbox of a 256 MiB kernel heap, and one capsule could halt
 * the machine. The tests below hold the budget to its caps under every
 * sequence of sends and receives, and pin the choice of a per-inbox share
 * over a cap on all a sender has waiting.
 */

#[path = "../../../../src/ipc/nonos_inbox/budget.rs"]
pub mod budget;
mod tests;
