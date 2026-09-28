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

//! `MkForeignInterrupt`: stopping a guest thread that is running its own code.
//!
//! A signal is delivered by answering a parked call with a handler to enter,
//! so a thread that makes no call never receives one. A supervisor marks such
//! a thread here. At the next timer tick that interrupts it in user mode the
//! kernel parks it with its whole register file, as if it had made a call
//! numbered `NR_INTERRUPTED`, and hands that to the supervisor. The answer is
//! a handler to enter, or anything else to run on exactly where it was. The
//! kernel stops and holds the thread; what it is stopped for is the
//! supervisor's.

mod call;
mod marks;
mod tick;
mod tick_frame;

pub use call::sys_foreign_interrupt;
pub(super) use marks::{forget, on_call};
pub use tick::on_user_tick;
pub use tick_frame::WORDS as TICK_FRAME_WORDS;
