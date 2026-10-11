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

//! The vendor table's source (src/vendor/table.rs), one row a program:
//! its store path, the BLAKE3 of its file from the Linux userland build
//! (NONOS_LINUX_USERLAND, `/bin/<name>` read as `<name>` there), and its
//! policy in this crate's own encoding. Without a userland build the file is
//! still checked line by line and the table is empty; a program it names
//! that is not there fails the build.

use std::fmt::Write;
use std::path::Path;
use std::{env, fs};

use crate::line::parse;
use crate::policy::LinuxAppPolicy;
use crate::unlisted::unlisted;

type Row = (String, [u8; 32], [u8; crate::layout::LEN]);

pub fn write(policies: &Path, to: &Path) -> Result<(), String> {
    let text = fs::read_to_string(policies).map_err(|e| format!("{}: {e}", policies.display()))?;
    let userland = env::var_os("NONOS_LINUX_USERLAND");
    let mut rows: Vec<Row> = Vec::new();
    for line in text.lines() {
        let (Some(l), Some(dir)) = (parse(line)?, &userland) else { continue };
        let at = Path::new(dir).join(&l.store_path["/bin/".len()..]);
        println!("cargo:rerun-if-changed={}", at.display());
        let elf = fs::read(&at).map_err(|e| format!("{}: {e}", at.display()))?;
        let program = *blake3::hash(&elf).as_bytes();
        let policy = LinuxAppPolicy { mask: l.mask, syscall_set: l.calls, ..unlisted(program) };
        let row = (l.store_path.to_string(), program, policy.encode());
        if rows.iter().any(|r| r.0 == row.0 || (r.1 == row.1 && r.2 != row.2)) {
            return Err(format!("{}: named twice, or one program in two boxes", row.0));
        }
        rows.push(row);
    }
    fs::write(to, source(&rows)).map_err(|e| format!("{}: {e}", to.display()))
}

fn source(rows: &[Row]) -> String {
    let mut out = String::from("&[\n");
    for (store_path, program_blake3, policy) in rows {
        let _ = writeln!(
            out,
            "    VendorRow {{ store_path: {store_path:?}, program_blake3: {program_blake3:?}, \
             policy: {policy:?} }},"
        );
    }
    out.push(']');
    out
}
