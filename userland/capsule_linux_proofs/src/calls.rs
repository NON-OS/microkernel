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

//! The capsule's pure call code, mounted from its tree as it ships: the
//! signal frame built and read back, the arithmetic of the timers that end in
//! a signal, and the reading of exec's shebang line. None of it names the
//! capsule's crate, so it runs here without a guest.

#[path = "../../capsule_linux/src/linux/call/sigframe.rs"]
pub mod sigframe;

#[path = "../../capsule_linux/src/linux/call/sigframe_build.rs"]
pub mod sigframe_build;

#[path = "../../capsule_linux/src/linux/call/sigframe_read.rs"]
pub mod sigframe_read;

#[path = "../../capsule_linux/src/linux/guest/sigtimer.rs"]
pub mod sigtimer;

#[path = "../../capsule_linux/src/linux/guest/sigtimer_rearm.rs"]
pub mod sigtimer_rearm;

#[path = "../../capsule_linux/src/linux/call/spawn/exec_shebang.rs"]
pub mod exec_shebang;

/* The load average's arithmetic: file code, not a call, but as pure. */
#[path = "load/mod.rs"]
pub mod loadavg;
