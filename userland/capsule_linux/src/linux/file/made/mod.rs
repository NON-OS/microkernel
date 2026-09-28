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
 * The trees the personality makes, /dev, /proc and /sys, and what
 * they read of the family.
 */

pub(super) mod boot_id;
pub(super) mod dev;
pub(super) mod exe;
pub(super) mod exe_image;
pub(super) mod fdopen;
pub(super) mod need;
pub(super) mod proc;
pub(super) mod synth;
pub(super) mod synth_ops;
pub(super) mod sys;
pub(super) mod view;

pub use exe::{of as exe_of, Exe};
pub use exe_image::record_image;
pub use need::needs_view;
pub(crate) use proc::mounts;
pub use proc::open_fds;
pub use view::{lend as lend_view, with as view_with, Proc, View};
