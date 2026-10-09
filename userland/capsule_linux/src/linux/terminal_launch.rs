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

//! Reading the program the terminal's `linux` command names, and saying
//! what stopped it when it cannot be read: what the store answered, never a
//! guess at why.

use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;

use super::launch::Launch;
use crate::linux::file::{key, store_read, store_stat};
use crate::linux::guest::Links;
use crate::linux::say::say;

pub(super) fn launch(program: &[u8], args: Vec<Vec<u8>>, max: u32) -> Option<Launch> {
    /* wait_settled says so itself when the store never settles. */
    if !super::settle::wait_settled() {
        return None;
    }
    let links = Links::try_load();
    let (named, path) = super::terminal_path::find(program, links.as_ref().ok());
    /* Through a link, argv[0] is the link's name, as an execve of it gives. */
    let argv0 = (path != named).then(|| named.clone());
    let why = match store_read(&key(&path), max) {
        Ok(bytes) => return Some(Launch { argv0, ..Launch::store(path, bytes, args) }),
        Err(why) => why,
    };
    /* A bare name was looked for along the whole PATH, not in one place. */
    let shown = if program.contains(&b'/') { named.clone() } else { program.to_vec() };
    let line = match store_stat(&key(&path)) {
        Ok((_, true)) => format!("linux: {} is a directory\n", text(&path)),
        Ok((size, false)) => {
            format!("linux: {} is there ({size} bytes) but unread: {why}\n", text(&path))
        }
        Err(_) => match nonos_app_skeleton::clients::vfs::store_status() {
            Ok(0) => match built_in_for(program, args) {
                Some(launch) => return Some(launch),
                None if program.contains(&b'/') => {
                    format!("linux: {}: no such file in the Linux tree\n", text(&shown))
                }
                None => format!(
                    "linux: {}: command not found (looked in {} and BusyBox's programs; \
                     `linux sh` then `ls /usr/bin` lists what is here)\n",
                    text(&shown),
                    searched()
                ),
            },
            Ok(code) => format!(
                "linux: cannot find {}: {} (store status {code})\n",
                text(&shown),
                super::store_why::store_why(code)
            ),
            Err(e) => {
                format!("linux: cannot find {}: the store does not answer ({e})\n", text(&shown))
            }
        },
    };
    say(line.as_bytes());
    if let (Err(why), Ok(_)) = (links, store_stat(&key(Links::TABLE))) {
        say(format!("linux: link table {} unread: {why}\n", text(Links::TABLE)).as_bytes());
    }
    None
}

/// The built-in BusyBox for `program` when the Linux tree has no such file.
/// It is part of this capsule's own image, measured by the manifest that
/// admitted the capsule, so it needs no proof from the store.
fn built_in_for(program: &[u8], args: Vec<Vec<u8>>) -> Option<Launch> {
    let name = super::built_in::serves(program)?;
    let argv0 = (name != b"busybox").then(|| name.to_vec());
    Some(Launch { args, argv0, ..super::built_in::built_in() })
}

fn text(path: &[u8]) -> String {
    String::from_utf8_lossy(path).into_owned()
}

/// The PATH a bare name was looked for along, as the guest's environment
/// gives it.
fn searched() -> String {
    let env = crate::linux::env::default();
    let path = env.iter().find_map(|v| v.strip_prefix(b"PATH=")).unwrap_or(b"/bin");
    text(path)
}
