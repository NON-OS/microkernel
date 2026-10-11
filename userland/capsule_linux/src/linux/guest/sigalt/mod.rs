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

//! A thread's alternate signal stack, kept as Linux's do_sigaltstack keeps
//! it: `[ss_sp, ss_flags, ss_size]`, the flags as the guest set them, and
//! no stack at all whenever the size is zero. Pure, so the host proofs hold
//! what sigaltstack reports, what it accepts, and when a thread is on it.

pub mod call;
mod change;
pub mod stack;

pub use call::sigaltstack;
pub use change::{change, entered};
pub use stack::{on_stack, NONE};
