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

use std::collections::BTreeSet;
use std::fs;
use std::path::Path;

use super::system_names::{is_reserved_form, is_system_name};

/// What this proof reads; tools/nix/inputs.py hands the build every match.
const CAPSULE_MKS: &str = "userland/*/Capsule.mk";

// The service, reply and instance names Capsule.mk files declare, split by
// whether the capsule's namespace is the system's own.
fn declared() -> (BTreeSet<String>, BTreeSet<String>) {
    let (mut system, mut publisher, mut read) = (BTreeSet::new(), BTreeSet::new(), 0);
    let userland = Path::new(env!("CARGO_MANIFEST_DIR")).join("..");
    for dir in fs::read_dir(&userland).expect("userland") {
        let Ok(raw) = fs::read_to_string(dir.expect("entry").path().join("Capsule.mk")) else {
            continue;
        };
        read += 1;
        // Make joins a line ending in a backslash with the next.
        let text = raw.replace("\\\n", " ");
        let ns = field(&text, "CAPSULE_NAMESPACE").next().expect("a namespace");
        let own = ns == "systems.nonos" || ns.starts_with("systems.nonos.");
        let eps = field(&text, "CAPSULE_SERVICE_ENDPOINT")
            .chain(field(&text, "CAPSULE_REPLY_ENDPOINT"))
            .chain(field(&text, "CAPSULE_INSTANCE_ENDPOINTS"));
        for ep in eps.flat_map(|v| v.split_whitespace().map(str::to_string).collect::<Vec<_>>()) {
            let name = ep.splitn(3, ':').nth(2).expect("kind:port:name").to_string();
            let set = if own { &mut system } else { &mut publisher };
            set.insert(name);
        }
    }
    // An empty view fails as itself, never as drift or a vacuous pass (L61).
    assert!(read >= 50, "{read} of {CAPSULE_MKS} under {userland:?}: the build lacks them");
    (system, publisher)
}

fn field<'a>(text: &'a str, key: &'a str) -> impl Iterator<Item = String> + 'a {
    text.lines().filter_map(move |l| {
        let (k, v) = l.split_once(":=")?;
        (k.trim() == key).then(|| v.trim().to_string())
    })
}

#[test]
fn the_list_is_every_system_capsules_names_and_no_others() {
    let listed: BTreeSet<String> =
        include_str!("../../../../src/services/registry/system_names.txt")
            .lines()
            .filter(|l| !l.is_empty())
            .map(str::to_string)
            .collect();
    let (system, publisher) = declared();
    assert_eq!(listed, system);
    let free = |n: &&String| !is_system_name(n) && !is_reserved_form(n);
    assert!(publisher.iter().all(|n| free(&n)), "a publisher example collides");
}
