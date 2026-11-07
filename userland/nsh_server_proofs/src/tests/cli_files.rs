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

//! The files `nsh` reads, written as an operator hands them to a user: the
//! user's key in nsh-enrol's secret file format, the fleet authority's
//! public key, the accepted releases, and the base grant.

use crate::kit::attest_build::keys::{hybrid, AUTHORITY};
use crate::kit::host::{built, HOST_NAME};
use crate::kit::roles_build::grant::GrantB;
use crate::kit::roles_build::keys::{key, K};
use std::path::{Path, PathBuf};

use super::cli_releases::hex;

/// A fresh directory with every file, and `nsh`'s arguments for `addr`.
pub fn files(name: &str, addr: &str, grant: &GrantB) -> (PathBuf, Vec<String>) {
    let d = std::env::temp_dir().join(format!("nsh-cli-{name}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&d);
    std::fs::create_dir_all(d.join("records")).expect("a directory");
    let seed = nonos_hash::sha256(&[b'n', b's', b'h', b'-', b'r', b'o', b'l', b'e', K::User as u8]);
    let u = key(K::User);
    let file = [&b"NSHA\x01\x00\x00\x00"[..], &seed, &u.ml_pk, u.ml_sk.as_bytes()].concat();
    let a = hybrid(AUTHORITY);
    let releases = super::cli_releases::releases();
    write(&d, "user.secret", &file);
    write(&d, "authority.public", &[&a.ed.public[..], &a.ml_public[..]].concat());
    write(&d, "releases", releases.as_bytes());
    write(&d, "grant", &grant.bytes());
    let args = args(&d, addr);
    (d, args)
}

/// `nsh`'s arguments for `addr`, naming the files in `d`.
pub fn args(d: &Path, addr: &str) -> Vec<String> {
    let p = |f: &str| d.join(f).to_string_lossy().into_owned();
    let args = [
        "--connect",
        addr,
        "--host",
        std::str::from_utf8(HOST_NAME).expect("name"),
        "--fleet",
        &hex(&built().fleet),
        "--authority",
        &p("authority.public"),
        "--releases",
        &p("releases"),
        "--key",
        &p("user.secret"),
        "--grant",
        &p("grant"),
        "--records",
        &p("records"),
    ];
    args.iter().map(|s| s.to_string()).collect()
}

fn write(d: &Path, name: &str, b: &[u8]) {
    std::fs::write(d.join(name), b).expect(name);
}
