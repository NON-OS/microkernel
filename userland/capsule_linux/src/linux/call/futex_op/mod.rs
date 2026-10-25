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

//! What a futex call asks for, decoded and checked as Linux's do_futex
//! checks it before any word is read, and the counts it carries as Linux
//! reads them. Pure, so the host proofs hold every refusal.

mod cmd;
mod counts;

pub use cmd::{decode, Cmd};
pub use counts::{requeue_counts, wake_count};
