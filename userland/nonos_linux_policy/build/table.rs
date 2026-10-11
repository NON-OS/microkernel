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
//! `<name>` there); without that build it is still checked line by line.
//! On a guest test image, the list NONOS_LINUX_GUEST_POLICIES names each
//! test guest with its `file=` (userland/linux_guests/GuestPolicies.mk).
//! A named file that is not there fails the build.

use std::path::{Path, PathBuf};
use std::{env, fs};

use super::line::{parse, Line};
use super::source::{source, Row};
use crate::policy::LinuxAppPolicy;
use crate::unlisted::unlisted;

pub fn write(policies: &Path, to: &Path) -> Result<(), String> {
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
    fs::write(to, source(&rows)).map_err(|e| format!("{}: {e}", to.display()))
}

fn read(list: &Path) -> Result<String, String> {
    println!("cargo:rerun-if-changed={}", list.display());
    fs::read_to_string(list).map_err(|e| format!("{}: {e}", list.display()))
}

fn lines(text: &str) -> Result<Vec<Line<'_>>, String> {
    text.lines().filter_map(|l| parse(l).transpose()).collect()
}

fn add(rows: &mut Vec<Row>, l: &Line<'_>, at: PathBuf) -> Result<(), String> {
    println!("cargo:rerun-if-changed={}", at.display());
    let elf = fs::read(&at).map_err(|e| format!("{}: {e}", at.display()))?;
    let program = *blake3::hash(&elf).as_bytes();
    let policy = LinuxAppPolicy { mask: l.mask, syscall_set: l.calls, ..unlisted(program) };
    let row = (l.store_path.to_string(), program, policy.encode());
    if rows.iter().any(|r| r.0 == row.0 || (r.1 == row.1 && r.2 != row.2)) {
        return Err(format!("{}: named twice, or one program in two boxes", row.0));
    }
    rows.push(row);
    Ok(())
}
