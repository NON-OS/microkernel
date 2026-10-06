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
 * The family's load average, measured. Linux averages the number of tasks
 * running or waiting to run, sampled every five seconds and decayed by
 * fixed factors for one, five and fifteen minutes. The kernel does not say
 * how long a thread waited for the CPU, so this averages what it does
 * say: the share of each period the family's threads ran, from their
 * ticks. A family whose threads wait for a CPU another holds reads lower
 * than Linux would show.
 *
 * Nothing samples between reads: at a read, the periods since the last
 * one each get the share the family ran over all of them.
 */

mod average;
mod state;

pub use average::{averages, text};
pub use state::exited;
