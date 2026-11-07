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

//! Running the command line in a fresh directory, and the fleet every test
//! starts from: a role authority, an approver and a user, each made by
//! `keygen`, and a table of the shipped roles they sign.

use std::path::{Path, PathBuf};
use std::process::{Command, Output};

pub const FLEET: &str = "f1f1f1f1f1f1f1f1f1f1f1f1f1f1f1f1";

pub fn run(dir: &Path, args: &[&str]) -> Output {
    let bin = env!("CARGO_BIN_EXE_nsh-roles");
    Command::new(bin).current_dir(dir).args(args).output().expect("runs")
}

pub fn ok(dir: &Path, args: &[&str]) -> String {
    let o = run(dir, args);
    assert!(o.status.success(), "{args:?}: {}", said(&o));
    said(&o)
}

pub fn fresh(name: &str) -> PathBuf {
    let d = std::env::temp_dir().join(format!("nsh-roles-{name}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&d);
    std::fs::create_dir_all(&d).expect("a directory");
    d
}

pub fn said(o: &Output) -> String {
    String::from_utf8_lossy(&o.stderr).into_owned() + &String::from_utf8_lossy(&o.stdout)
}

/// A fleet in `dir`; the table's anchor, the key id hosts pin.
pub fn fleet(dir: &Path) -> String {
    for name in ["auth", "approver", "user", "other"] {
        ok(dir, &["keygen", "--out", ".", "--name", name]);
    }
    let said = ok(
        dir,
        &[
            "table",
            "--fleet",
            FLEET,
            "--serial",
            "1",
            "--max-age-hours",
            "24",
            "--authority",
            "auth.public",
            "--approver",
            "approver.public",
            "--key",
            "auth.secret",
            "--out",
            "table",
        ],
    );
    said.trim().strip_prefix("table anchor ").expect("an anchor").to_string()
}
