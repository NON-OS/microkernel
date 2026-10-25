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

mod drawing;
mod frame;
mod held;
mod hex;
pub(crate) mod hidpi;
pub(crate) mod init;
mod log;
mod marker;
mod report;
mod state;
mod window;

pub(crate) use drawing::{drawable, wait_quiet};
pub(crate) use held::{end_takeover, set_gone, set_held, stop_drawing, surrendered, taken};
pub(crate) use init::init_framebuffer;
pub(crate) use state::{boot_frame_len, framebuffer_phys, framebuffer_state, KernelFramebuffer};
pub(crate) use window::phys_window;
