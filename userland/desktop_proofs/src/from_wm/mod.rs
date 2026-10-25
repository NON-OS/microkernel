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

//! The window manager's own code: geometry, the window table, the z stack,
//! focus, and the lifecycle notification it writes.

// The window manager: geometry, the window table (assembled as in wm_proofs,
// whose file this reuses), the z stack and click to focus.
#[path = "../../../capsule_wm/src/geometry/mod.rs"]
pub mod geometry;

#[path = "../../../wm_proofs/src/window.rs"]
pub mod window;

#[path = "../../../capsule_wm/src/z_order/mod.rs"]
pub mod z_order;

#[path = "../../../capsule_wm/src/focus/mod.rs"]
pub mod focus;

// The window manager's lifecycle notification as it writes it, and as the
// shell reads it.
#[path = "../../../capsule_wm/src/protocol/notify.rs"]
pub mod wm_notify_encode;
