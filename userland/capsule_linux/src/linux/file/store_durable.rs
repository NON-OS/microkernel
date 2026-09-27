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

//! A store write that survives the next boot.
//!
//! The store is staged into RAM at boot, so a plain write is undone by
//! restarting. An installed program and the proof beside it are persisted as
//! they are written; nothing about them is trusted after the reboot, since
//! exec proves a program again every time it runs.

use nonos_app_skeleton::clients::vfs;
use nonos_libc::mk_getpid;

use super::root::Key;
use super::store::write;

pub fn write_durable(at: &Key, data: &[u8]) -> Result<(), &'static str> {
    write(at, data)?;
    // A file that landed but did not persist is reported, not kept silently
    // in RAM as if it had been installed.
    vfs::persist(mk_getpid(), at.as_bytes())
}
