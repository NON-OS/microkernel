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

//! A window painted through its swapchain against the single surface it
//! replaced. The old path drew a partial repaint off the surface and wrote
//! the damaged rows back; the new draws into a buffer the compositor
//! released, the content area alone unless more of that buffer is out of
//! date, and presents the damage. Run side by side, the buffer the
//! compositor shows equals the old surface after every frame, and no
//! frame writes a buffer the compositor shows or holds.

mod changed_tests;
mod new_path;
mod rig;
mod shown_tests;
mod whole_tests;
