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

//! Which program to run.

use alloc::vec::Vec;

use super::launch::Launch;
use super::source_named::named;
use super::source_run::run;

pub(super) const MAX_IMAGE: u32 = 64 << 20;

/// The program, where it came from, and what it is given. A run starts a
/// shipped tier, or an installed package's recorded program, or nothing:
/// the built-in program would start something the person did not ask for.
pub fn source() -> Option<Launch> {
    let store = Launch::store;
    if let Some(asked) = super::terminal::requested(MAX_IMAGE) {
        return asked;
    }
    if let Some((name, mode)) = super::request::run_request() {
        return run(&name, mode);
    }
    if let Some((path, bytes)) = named(MAX_IMAGE) {
        return Some(store(path, bytes, Vec::new()));
    }
    if let Some((path, bytes, args)) = super::boot_guest::boot_guest(MAX_IMAGE) {
        return Some(store(path, bytes, args));
    }
    /* Started with nothing asked of it, as the desktop starts it at boot.
     * Running the built-in program here ran BusyBox with no arguments,
     * which printed its whole usage into the boot log. */
    super::start_say::routine(b"[LINUX] personality ready, nothing asked to run\n");
    nonos_libc::mk_exit(0)
}
