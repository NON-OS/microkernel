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

//! The personality's policy files that decide without a syscall, mounted
//! as they ship.

/* Which policy binds a proved program, and the box it runs in after exec. */
#[path = "../../../capsule_linux/src/linux/app/choose.rs"]
pub mod choose;
#[path = "../../../capsule_linux/src/linux/app/identity.rs"]
pub mod identity;
#[path = "../../../capsule_linux/src/linux/app/identity_exec.rs"]
mod identity_exec;
/* The same identity held inside a `linux --box` box. */
#[path = "../../../capsule_linux/src/linux/app/identity_box.rs"]
mod identity_box;
/* Where a guest may reach out to: its lists, through an exec. */
#[path = "../../../capsule_linux/src/linux/app/reach.rs"]
pub mod reach;
/* What a box allows: the call set, the network, before anything is served. */
#[path = "../../../capsule_linux/src/linux/app/rules.rs"]
pub mod rules;
