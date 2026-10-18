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

//! The vendor table's rows (src/vendor/table.rs): each program's store path,
//! the BLAKE3 of its file, and its policy in this crate's own encoding.
//!
//! Two lists feed it. policies.txt names the vendor's programs, whose files
//! are in the Linux userland build (NONOS_LINUX_USERLAND; `/bin/<name>` is
//! `<name>` there); without it the file is checked and binds nothing.
//! On a guest test image, the list NONOS_LINUX_GUEST_POLICIES names each
//! test guest with its `file=` (userland/linux_guests/GuestPolicies.mk).
//! A named file that is not there fails the build.

use std::path::{Path, PathBuf};
use std::{env, fs};

use super::line::{parse, Line};
use super::row_of::add;
use super::source::{source, Row};

pub fn write(policies: &Path, to: &Path) -> Result<usize, String> {
    let mut rows: Vec<Row> = Vec::new();
    let userland = env::var_os("NONOS_LINUX_USERLAND");
    for l in lines(&read(policies)?)? {
        if l.file.is_some() {
            return Err(format!("{}: file= is for test guests", l.store_path));
        }
        if let Some(dir) = &userland {
            add(&mut rows, &l, Path::new(dir).join(&l.store_path["/bin/".len()..]))?;
        }
    }
    if let Some(guests) = env::var_os("NONOS_LINUX_GUEST_POLICIES") {
        for l in lines(&read(Path::new(&guests))?)? {
            let file = l.file.ok_or_else(|| format!("{}: no file=", l.store_path))?;
            add(&mut rows, &l, PathBuf::from(file))?;
        }
    }
    fs::write(to, source(&rows)).map(|_| rows.len()).map_err(|e| format!("{}: {e}", to.display()))
}

fn read(list: &Path) -> Result<String, String> {
    println!("cargo:rerun-if-changed={}", list.display());
    fs::read_to_string(list).map_err(|e| format!("{}: {e}", list.display()))
}

fn lines(text: &str) -> Result<Vec<Line<'_>>, String> {
    text.lines().filter_map(|l| parse(l).transpose()).collect()
}
