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

mod cover;
mod cover_rule;
mod hold;
mod info;
mod register;
pub(crate) mod rule;
mod sink;

pub(in crate::hardware::broker) use cover::covers;
pub(in crate::hardware::broker) use hold::{admitted, released, say};
pub use info::{info_for, BootFbInfoError};
pub use register::register_boot_framebuffer;
pub(in crate::hardware::broker) use register::{device, is_boot_framebuffer};
pub(in crate::hardware::broker) use sink::get as sink;
