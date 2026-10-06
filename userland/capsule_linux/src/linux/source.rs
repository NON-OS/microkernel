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

use crate::linux::file::{family::choose, key, store_read};

use super::launch::Launch;
use super::source_named::named;

const MAX_IMAGE: u32 = 64 << 20;

/// The program, where it came from, and what it is given. A run starts a
/// shipped tier, or an installed package's recorded program, or nothing:
/// the built-in program would start something the person did not ask for.
pub fn source() -> Option<Launch> {
    let store = Launch::store;
    if let Some(asked) = super::terminal::requested(MAX_IMAGE) {
        return asked;
    }
    if let Some((name, mode)) = super::request::run_request() {
        /* Before the guest's first byte, so a terminal run is private from it. */
        super::console::enter(mode);
        /* A tier's program, or a package's, is read from the store: until
         * the VFS has finished loading it from the stick, a program that is
         * there reads as one that is not. */
        if !super::settle::wait_settled() {
            return refused(b"linux: the package store has not finished loading; try again", "");
        }
        let pkg = choose(&name);
        match super::install::launch(pkg, mode) {
            Some(Ok((path, bytes, args))) => {
                crate::linux::file::machine::show();
                return Some(store(path, bytes, args));
            }
            Some(Err(why)) => {
                return refused(b"linux: this tier's program could not be read: ", why)
            }
            None => {}
        }
        let Some(path) = super::install::recorded(pkg) else {
            return refused(b"linux: nothing installed under that name", "");
        };
        return match store_read(&key(&path), MAX_IMAGE) {
            Ok(bytes) => Some(store(path, bytes, Vec::new())),
            Err(why) => refused(b"linux: the installed program could not be read: ", why),
        };
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

/// Say why nothing starts, once, on the run's terminal or else in the log.
fn refused(what: &[u8], why: &str) -> Option<Launch> {
    let line = [what, why.as_bytes(), b"\n"].concat();
    match super::console::attached() {
        true => super::console::say(&line),
        false => super::say::say(&line),
    }
    None
}
