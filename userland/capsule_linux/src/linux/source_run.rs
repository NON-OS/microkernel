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

//! A run asked for by name: a shipped tier's program, or an installed
//! package's, read from the store; or why neither starts.

use alloc::vec::Vec;

use crate::linux::file::{family::choose, key, store_read};
use crate::linux::run_mode::Mode;

use super::launch::Launch;
use super::source::MAX_IMAGE;

/// The program the run of `name` in `mode` starts, or None, said.
pub(super) fn run(name: &str, mode: Mode) -> Option<Launch> {
    let store = Launch::store;
    /* Before the guest's first byte, so a terminal run is private from it. */
    super::console::enter(mode);
    /* A tier's program, or a package's, is read from the store: until
     * the VFS has finished loading it from the stick, a program that is
     * there reads as one that is not. */
    if !super::settle::wait_settled() {
        return refused(b"linux: the package store has not finished loading; try again", "");
    }
    let pkg = choose(name);
    match super::install::launch(pkg, mode) {
        Some(Ok((path, bytes, args))) => {
            crate::linux::file::machine::show();
            return Some(store(path, bytes, args));
        }
        Some(Err(why)) => return refused(b"linux: this tier's program could not be read: ", why),
        None => {}
    }
    let Some(path) = super::install::recorded(pkg) else {
        return refused(b"linux: nothing installed under that name", "");
    };
    match store_read(&key(&path), MAX_IMAGE) {
        Ok(bytes) => Some(store(path, bytes, Vec::new())),
        Err(why) => refused(b"linux: the installed program could not be read: ", why),
    }
}

/// Say why nothing starts, once, on the run's terminal or else in the log.
fn refused(what: &[u8], why: &str) -> Option<Launch> {
    let line = [what, why.as_bytes(), b"\n"].concat();
    match super::console::attached() {
        true => super::console::say(&line),
        false => super::say::say(&line),
    }
    None
}
