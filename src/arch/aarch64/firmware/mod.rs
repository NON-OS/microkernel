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

//! The Raspberry Pi's VideoCore firmware, reached through its mailbox:
//! clock rates the device tree does not give, and the framebuffer the
//! firmware drives the display from.

mod call;
mod clock;
mod framebuffer;
mod message;
mod regs;
mod state;

pub use clock::{clock_rate, CLOCK_EMMC2};
pub use framebuffer::framebuffer;
pub use state::describe;
