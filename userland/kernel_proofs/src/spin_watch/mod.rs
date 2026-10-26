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
 * When a lock_responsive wait is reported, and when it is called a re-take by
 * its own holder: the kernel's own rule file, and the process kernel stack
 * size the re-take window is drawn from.
 */

#[path = "../../../../src/smp/spin_verdict.rs"]
pub mod spin_verdict;
#[path = "../../../../src/smp/spin_watch.rs"]
pub mod spin_watch;
#[cfg(test)]
mod tests;
#[path = "../../../../src/process/userspace/constants.rs"]
pub mod user_constants;
#[cfg(test)]
mod verdict_tests;
