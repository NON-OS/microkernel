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

//! A word the Terminal's `linux` command names that is no program on the
//! PATH but an installed package: its recorded program, in its family's
//! tree, as the store's Open of a terminal program hands it over
//! (terminal_hand.rs) and the store tells the person to type, `linux
//! kali.<package>` for a Debian one.

use alloc::format;
use alloc::vec::Vec;

use super::super::launch::Launch;
use super::find::text;
use crate::linux::file::family::{choose, run_name};
use crate::linux::file::{key, store_read};
use crate::linux::say::say;

/// The program `word` installed as, read from the store, with `args`: Ok
/// with it, or Ok(None) when it was installed and cannot be read, said; or
/// `args` back when nothing is installed under that word, the family left
/// as Alpine's for the PATH's lookup.
pub(super) fn installed(
    word: &[u8],
    args: Vec<Vec<u8>>,
    max: u32,
) -> Result<Option<Launch>, Vec<Vec<u8>>> {
    let Some(name) = core::str::from_utf8(word).ok().and_then(run_name) else { return Err(args) };
    let pkg = choose(&name);
    let Some(path) = super::super::install::recorded(pkg) else {
        choose("");
        return Err(args);
    };
    // Its packs first, as a store run's are: the program is read from them.
    crate::linux::file::pack::run_closure(super::super::install::closure(pkg));
    match store_read(&key(&path), max) {
        Ok(bytes) => Ok(Some(Launch::store(path, bytes, args))),
        Err(why) => {
            say(format!(
                "linux: {} is installed but its program {} is unread: {why}\n",
                text(word),
                text(&path)
            )
            .as_bytes());
            Ok(None)
        }
    }
}
