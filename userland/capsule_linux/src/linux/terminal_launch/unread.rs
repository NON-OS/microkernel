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

//! What the terminal is told when the program its `linux` command names
//! cannot be read: what the store answered, never a guess at why; or the
//! built-in program it names after all.

use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;

use super::super::launch::Launch;
use super::find::{built_in_for, searched, text};
use crate::linux::file::{key, store_stat};

/// The line for `program`, looked for at `path` and shown as `shown`, that
/// the store would not give (`why`); or the built-in program it names.
pub(super) fn unread(
    program: &[u8],
    args: Vec<Vec<u8>>,
    (path, shown): (&[u8], &[u8]),
    why: &str,
) -> Result<Launch, String> {
    let line = match store_stat(&key(path)) {
        Ok((_, true)) => format!("linux: {} is a directory\n", text(path)),
        Ok((size, false)) => {
            format!("linux: {} is there ({size} bytes) but unread: {why}\n", text(path))
        }
        Err(_) => match nonos_app_skeleton::clients::vfs::store_status() {
            Ok(0) => match built_in_for(program, args) {
                Some(launch) => return Ok(launch),
                None if program.contains(&b'/') => {
                    format!("linux: {}: no such file in the Linux tree\n", text(shown))
                }
                None => format!(
                    "linux: {}: command not found (looked in {} and BusyBox's programs; \
                     `linux sh` then `ls /usr/bin` lists what is here)\n",
                    text(shown),
                    searched()
                ),
            },
            Ok(code) => format!(
                "linux: cannot find {}: {} (store status {code})\n",
                text(shown),
                super::super::store_why::store_why(code)
            ),
            Err(e) => {
                format!("linux: cannot find {}: the store does not answer ({e})\n", text(shown))
            }
        },
    };
    Err(line)
}
