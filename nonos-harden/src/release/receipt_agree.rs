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

//! Where a receipt and the record disagree, and what a receipt that agrees
//! still leaves uncovered.

use super::receipt::Receipt;
use super::record::{Item, Record};
use crate::hash::short;
use std::collections::BTreeSet;

pub fn disagreements(r: &Receipt, rec: &Record) -> Vec<String> {
    let roots = [
        (r.embedded, "the seal found a root it measured not embedded"),
        (r.policy_in_kernel, "the capsule policy root is not embedded in the kernel"),
        (r.attest_root != Some(false), "the kernel attestation root did not check"),
    ];
    let mut bad: Vec<String> =
        roots.iter().filter(|(ok, _)| !ok).map(|(_, why)| why.to_string()).collect();
    let sealed = digests(rec);
    for (name, sha) in &r.artifacts {
        match sha {
            None => bad.push(format!("{name} was missing when the seal measured it")),
            Some(s) if !sealed.contains(s.as_str()) => {
                bad.push(format!("{name}: sha256 {}… is not among the sealed bytes", short(s)))
            }
            Some(_) => {}
        }
    }
    bad
}

pub fn limits(r: &Receipt) -> Vec<String> {
    let mut limits = Vec::new();
    if r.attest_root.is_none() {
        limits.push("the receipt measured no kernel attestation root".to_string());
    }
    if r.dirty {
        let c = short(&r.commit);
        limits.push(format!(
            "the sealing tree held changes beyond {c} (the seal stages its trust set first)"
        ));
    }
    limits
}

/// Every digest the record seals, files and the files of directories alike.
fn digests(rec: &Record) -> BTreeSet<&str> {
    let mut all = BTreeSet::new();
    for s in &rec.sealed {
        match &s.item {
            Item::File(h) => {
                all.insert(h.as_str());
            }
            Item::Dir(files) => all.extend(files.iter().map(|(_, h)| h.as_str())),
        }
    }
    all
}
