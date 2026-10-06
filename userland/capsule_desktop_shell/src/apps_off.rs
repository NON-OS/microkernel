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

/*
 * The apps first-boot setup turned off, as the kernel holds them. The kernel
 * spawns none of them, at boot or for a click, so every way in (dock,
 * Launchpad, the Go menu) shows each as off and says why it opens nothing.
 */

mod dim;
mod mask;
mod open;
mod qwen;

pub use dim::dim;
pub use mask::is_off;
pub use open::{expect, open, request, say_default, toast_failed};
