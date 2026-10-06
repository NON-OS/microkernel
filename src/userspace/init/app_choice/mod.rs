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
 * The apps first-boot setup turned off. Setup's exit status carries them;
 * init records them before it starts the desktop, and from then on no app
 * turned off is spawned, at boot or on demand. Only spawns are withheld:
 * every check an app that does start goes through is unchanged.
 */

mod bits;
mod gate;
mod names;
mod present;
mod profile;

#[cfg(feature = "microkernel-setup-wizard")]
pub(crate) use gate::choose;
pub(crate) use gate::off;
pub(crate) use names::{capsule_off, linux_off, tool_off, window_off};
pub(crate) use present::PRESENT;
pub(crate) use profile::apply as apply_profile;
