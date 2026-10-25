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

//! Running the tool over files in a fresh directory.

use crate::kit::attest::TestAttester;
use crate::kit::keys;
use crate::kit::record::record_doc;
use nsh_client::cli::run;
use std::path::{Path, PathBuf};

pub fn dir(name: &str) -> PathBuf {
    let d = std::env::temp_dir().join(format!("nsh_client_proofs_{name}_{}", std::process::id()));
    std::fs::create_dir_all(&d).unwrap();
    d
}

pub fn file(d: &Path, name: &str, bytes: &[u8]) -> String {
    let p = d.join(name);
    std::fs::write(&p, bytes).unwrap();
    p.to_string_lossy().into_owned()
}

pub fn args(list: &[&str]) -> Vec<String> {
    list.iter().map(|s| s.to_string()).collect()
}

pub fn hex(b: &[u8]) -> String {
    b.iter().map(|x| format!("{x:02x}")).collect()
}

pub const FLEET: &str = "f1f1f1f1f1f1f1f1f1f1f1f1f1f1f1f1";

/// `record` checked against the good document and authority, with `more`.
pub fn tool(d: &Path, record: &[u8], more: &[&str]) -> (i32, Vec<String>) {
    let rec = file(d, "record", record);
    let doc = file(d, "doc", &record_doc(&[0x7A; 64]));
    let key = file(d, "key", &keys::authority().public_bytes());
    let base = args(&[&rec, "--doc", &doc, "--fleet", FLEET, "--authority", &key]);
    let (code, text) = run(&[base, args(more)].concat(), &TestAttester::default());
    (code, text.lines().map(String::from).collect())
}
