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

//! The program the terminal's `linux` command names, read from the store or
//! found built in.

use alloc::format;
use alloc::vec::Vec;

use super::super::launch::Launch;
use super::find::text;
use super::unread::unread;
use crate::linux::file::{key, store_read, store_stat};
use crate::linux::guest::Links;
use crate::linux::say::say;

pub(in super::super) fn launch(program: &[u8], args: Vec<Vec<u8>>, max: u32) -> Option<Launch> {
    /* wait_settled says so itself when the store never settles. */
    if !super::super::settle::wait_settled() {
        return None;
    }
    let links = Links::try_load();
    let (named, path) = super::super::terminal_path::find(program, links.as_ref().ok());
    /* Through a link, argv[0] is the link's name, as an execve of it gives. */
    let argv0 = (path != named).then(|| named.clone());
    let why = match store_read(&key(&path), max) {
        Ok(bytes) => return Some(Launch { argv0, ..Launch::store(path, bytes, args) }),
        Err(why) => why,
    };
    /* A bare name was looked for along the whole PATH, not in one place. */
    let shown = if program.contains(&b'/') { named.clone() } else { program.to_vec() };
    let line = match unread(program, args, (&path, &shown), why) {
        Ok(launch) => return Some(launch),
        Err(line) => line,
    };
    say(line.as_bytes());
    if let (Err(why), Ok(_)) = (links, store_stat(&key(Links::TABLE))) {
        say(format!("linux: link table {} unread: {why}\n", text(Links::TABLE)).as_bytes());
    }
    None
}
