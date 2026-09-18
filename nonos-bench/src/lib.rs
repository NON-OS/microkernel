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

//! Measuring the primitives that define a microkernel, in cycles.
//!
//! The numbers this system publishes about itself have so far been wall clock:
//! how long a build took, how long a boot took. Those are the numbers that are
//! easy to take and the ones that say least. An IPC round trip is the operation
//! every capsule on this machine pays for, thousands of times a second, and
//! nobody has ever measured it.
//!
//! Three rules, and they are the reason this is a module rather than a loop
//! written at each call site.
//!
//! A mean hides everything worth knowing. The tail is where a scheduler misses
//! a wake and where a user notices, so a summary here is percentiles and a
//! maximum, never an average.
//!
//! The counter costs something to read, and at these magnitudes it is a large
//! fraction of what is being measured: reading it twice around an operation
//! that takes forty cycles reports the operation and the reading together.
//! Overhead is measured against an empty body and subtracted.
//!
//! The processor is free to reorder around an unserialised read, so a naive
//! `rdtsc` pair can close before the work it is timing has finished. Every read
//! here is fenced.

#![no_std]

mod overhead;
mod run;
mod sort;
mod summary;
mod tsc;

pub use overhead::Overhead;
pub use run::measure;
pub use sort::sort;
pub use summary::Summary;
pub use tsc::read_serialised;
