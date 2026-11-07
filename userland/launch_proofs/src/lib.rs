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

//! Host proofs for the capsule launch path.

#[cfg(test)]
#[path = "../../../src/kernel_core/process_spawn/capsule_spawn/launch_trace"]
mod launch_trace {
    pub(crate) mod line;
    pub(crate) mod phase;
    pub(crate) mod record;
    pub(crate) mod table;
}

#[cfg(test)]
mod trace_tests;

#[cfg(test)]
#[path = "../../../src/security/admission_cache"]
mod admission_cache {
    pub(crate) mod key;
    pub(crate) mod table;
}

#[cfg(test)]
mod cache_tests;

// The clock page: the kernel's layout and writer, libc's reader, and the
// kernel's own millisecond scaling the reader must agree with.
#[cfg(test)]
#[path = "../../../src/sys/clock/page/layout.rs"]
mod clock_layout;
#[cfg(test)]
#[path = "../../libc/src/time/clock_page.rs"]
mod clock_reader;
#[cfg(test)]
#[path = "../../../src/sys/clock/core/scale.rs"]
mod clock_scale;

#[cfg(test)]
mod clock_tests;
