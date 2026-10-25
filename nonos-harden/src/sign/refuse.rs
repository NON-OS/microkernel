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

//! A verdict is signed with the operator's own key, never with one of
//! NONOS's. A key file is refused by where it lies before it is opened (the
//! path followed through links first): under `nonos-bootloader/keys` or
//! `nonos-data`, or in the `.keys` of a NONOS checkout, where NONOS keeps
//! its private and published keys. Once read, it is refused again if its
//! public key is one the tree's trust set publishes.

use crate::release::read::read_regular;
use nonos_capsule_sign::algs::AlgId;
use std::fs;
use std::path::Path;

const KEYS: &str = "nonos-data/trust/keys";

pub fn by_place(path: &Path) -> Result<(), String> {
    let real = fs::canonicalize(path).map_err(|e| format!("{}: {e}", path.display()))?;
    let named = |a: &str, b: &str| real.ancestors().any(|d| d.ends_with(Path::new(a).join(b)));
    let checkout_keys = real.ancestors().any(|d| {
        d.file_name().is_some_and(|n| n == ".keys")
            && d.parent().is_some_and(|c| {
                c.join("nonos-bootloader").is_dir() && c.join("nonos-data").is_dir()
            })
    });
    let in_data = real.ancestors().any(|d| d.file_name().is_some_and(|n| n == "nonos-data"));
    match checkout_keys || in_data || named("nonos-bootloader", "keys") {
        true => {
            Err(format!("{} lies where NONOS keeps its own keys; sign with yours", real.display()))
        }
        false => Ok(()),
    }
}

pub fn by_public(tree: &Path, public: &[u8; 32]) -> Result<(), String> {
    let Ok(entries) = fs::read_dir(tree.join(KEYS)) else {
        return Ok(());
    };
    for p in entries.flatten().map(|e| e.path()) {
        // Read bounded and without following a link, as every file here is:
        // a FIFO or a vast file among the keys is skipped, never waited on.
        let bytes = read_regular(&p, 4096).unwrap_or_default();
        // In nonos-sign's NONOSPK1 file, or raw, as the kernel key is kept.
        let ed =
            bytes.strip_prefix(b"NONOSPK1").filter(|r| r.first() == Some(&(AlgId::Ed25519 as u8)));
        let headed =
            ed.is_some_and(|r| r.get(1..3) == Some(&[0, 32]) && r.get(3..) == Some(&public[..]));
        if headed || bytes == public[..] {
            let name = p.file_name().map(|n| n.to_string_lossy().into_owned()).unwrap_or_default();
            return Err(format!("that is NONOS's key {KEYS}/{name}; sign with yours"));
        }
    }
    Ok(())
}
