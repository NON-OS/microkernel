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
 * Per-launch phase timing: one `[LAUNCH]` serial line per capsule launch,
 * from the spawn call to the capsule's first frame. Built only with
 * `nonos-launch-trace`; without it every hook below is an empty inline
 * function and the spawn and syscall paths carry no trace of it.
 */

mod phase;

pub(crate) use phase::Phase;

#[cfg(feature = "nonos-launch-trace")]
mod line;
#[cfg(feature = "nonos-launch-trace")]
mod live;
#[cfg(feature = "nonos-launch-trace")]
mod record;
#[cfg(feature = "nonos-launch-trace")]
mod table;

#[cfg(feature = "nonos-launch-trace")]
pub(crate) use live::{on_exit, on_frame, on_surface, on_syscall, Trace};

#[cfg(not(feature = "nonos-launch-trace"))]
mod off;

#[cfg(not(feature = "nonos-launch-trace"))]
pub(crate) use off::{on_exit, on_frame, on_surface, on_syscall, Trace};
