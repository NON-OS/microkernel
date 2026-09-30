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

use crate::linux::file::family::choose;
use crate::linux::file::{key, store_read};

use super::launch::Launch;
use super::origin::Origin;
use super::source_named::named;

const MAX_IMAGE: u32 = 64 << 20;

/// The program, where it came from, and what it is given. A run starts a
/// shipped tier, or an installed package's recorded program, or nothing:
/// the built-in program would start something the person did not ask for.
pub fn source() -> Option<Launch> {
    let store = |path: Vec<u8>, bytes, args| Launch { path, bytes, origin: Origin::Store, args };
    if let Some((name, mode)) = super::request::run_request() {
        // Before the guest's first byte, so a terminal run is private from it.
        super::console::enter(mode);
        let pkg = choose(&name);
        if let Some((path, bytes, args)) = super::install::launch(pkg, mode) {
            crate::linux::file::machine::show();
            return Some(store(path, bytes, args));
        }
        let Some(path) = super::install::recorded(pkg) else {
            super::console::say(b"linux: nothing installed under that name\n");
            return None;
        };
        let bytes = store_read(&key(&path), MAX_IMAGE).ok()?;
        return Some(store(path, bytes, Vec::new()));
    }
    if let Some((path, bytes)) = named(MAX_IMAGE) {
        return Some(store(path, bytes, Vec::new()));
    }
    if let Some((path, bytes, args)) = super::boot_guest::boot_guest(MAX_IMAGE) {
        return Some(store(path, bytes, args));
    }
    Some(super::built_in::built_in())
}
