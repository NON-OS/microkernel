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

//! A family's home, seeded the first time it opens (seed_rule.rs): the
//! person's folders for GLib and Qt when the program is served them, then
//! whatever its image ships under /usr/share/nonos/home-seed.
//!
//! A seed never stops a program from starting: a home it could not fill is
//! said once on the serial line and the program runs with what landed.

use alloc::vec::Vec;

use nonos_app_skeleton::clients::vfs;
use nonos_libc::mk_getpid;

use super::seed_rule::{
    index, plant, Fault, Home, Planted, MOST_BYTES, USER_DIRS, USER_DIRS_LOCALE,
};
use super::seed_rule::{USER_DIRS_LOCALE_TEXT, USER_DIRS_TEXT};
use crate::linux::file::{resolve, store};

/// Where an image ships its seed, and the list of what is in it.
const SHIPPED: &[u8] = b"/usr/share/nonos/home-seed";
const INDEX: &[u8] = b"/usr/share/nonos/home-seed/INDEX";

/// The guest's /root, at its store path.
struct StoreHome(Vec<u8>);

impl StoreHome {
    fn at(&self, path: &[u8]) -> Vec<u8> {
        [&self.0[..], b"/", path].concat()
    }
}

impl Home for StoreHome {
    fn has(&mut self, path: &[u8]) -> Result<bool, Fault> {
        Ok(vfs::stat(mk_getpid(), &self.at(path)).is_ok())
    }

    fn mkdir(&mut self, path: &[u8]) -> Result<(), Fault> {
        let (pid, at) = (mk_getpid(), self.at(path));
        let _ = vfs::mkdir(pid, &at);
        vfs::dirstat(pid, &at).map(drop).map_err(|_| Fault::Refused)
    }

    fn write(&mut self, path: &[u8], bytes: &[u8]) -> Result<(), Fault> {
        vfs::write_file(mk_getpid(), &self.at(path), bytes).map_err(|why| match why {
            "no space left" => Fault::Full,
            _ => Fault::Refused,
        })
    }
}

/// Seeds the open home (home.rs), with the person's folders when the
/// program is served them (`user_files`).
pub fn seed(user_files: bool) {
    let Some(root) = super::home::root() else {
        return;
    };
    let mut home = StoreHome([&root[..], b"/root"].concat());
    let shipped = shipped();
    let mut files: Vec<(&[u8], &[u8])> = Vec::new();
    if user_files {
        files.push((USER_DIRS, USER_DIRS_TEXT));
        files.push((USER_DIRS_LOCALE, USER_DIRS_LOCALE_TEXT));
    }
    files.extend(shipped.iter().map(|(p, b)| (p.as_slice(), b.as_slice())));
    match plant(&mut home, &files) {
        Ok(Planted::Before | Planted::Now { .. }) => {}
        Err(Fault::Full) => {
            say(b"[LINUX] its home is full: seeded in part, the rest at the next start\n")
        }
        Err(Fault::Refused) => say(b"[LINUX] its home refused the seed: seeded in part\n"),
    }
}

/// The files the image ships for a new home, read whole and bounded; none
/// when there is no INDEX, or one that names anything out of bounds.
fn shipped() -> Vec<(Vec<u8>, Vec<u8>)> {
    let Ok(text) = store::read(&resolve::key(INDEX), MOST_BYTES as u32) else {
        return Vec::new();
    };
    let Some(paths) = index(&text) else {
        say(b"[LINUX] its image's home seed names a path out of bounds: not seeded\n");
        return Vec::new();
    };
    let (mut out, mut total) = (Vec::new(), 0usize);
    for p in paths {
        let at = [SHIPPED, b"/", p].concat();
        let left = MOST_BYTES.saturating_sub(total);
        match store::read(&resolve::key(&at), left as u32 + 1) {
            Ok(bytes) if bytes.len() <= left => {
                total += bytes.len();
                out.push((p.to_vec(), bytes));
            }
            _ => {
                say(b"[LINUX] its image's home seed is missing a file or past 1 MiB: not seeded\n");
                return Vec::new();
            }
        }
    }
    out
}

fn say(line: &[u8]) {
    let _ = nonos_libc::mk_debug(line.as_ptr(), line.len());
}
