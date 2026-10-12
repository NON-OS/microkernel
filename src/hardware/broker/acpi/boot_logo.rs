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

//! The firmware's boot logo (BGRT), as the x86 parser copied it at boot,
//! through the broker's one platform shim so the syscall layer names no
//! arch path.

use super::kernel::platform::fw;

pub use fw::parser::other::bgrt_entry::Bgrt;
pub use fw::parser::other::bmp_check::Bmp;
pub use fw::parser::other::{boot_logo, boot_logo_file};
