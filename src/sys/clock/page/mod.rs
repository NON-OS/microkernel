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

//! The clock page: one read-only page every capsule maps, from which its
//! libc reads the monotonic clock with the cycle counter and no syscall.
//! The kernel writes it (`place`); user code never can, and nothing on it
//! is executable. Layout and seqlock are here; libc keeps its own reader
//! (`userland/libc/src/time/clock_page.rs`), held to this one in
//! `launch_proofs` (`clock_tests.rs`).

pub mod layout;
pub mod mapping;
mod place;

pub use mapping::VA;
pub use place::{is_frame, map_into, mapped_in, refresh};
