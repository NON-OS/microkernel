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

use super::system_names::is_system_name;

// The service and reply names Capsule.mk files declare, split by whether the
// capsule's namespace is the system's own.
fn declared() -> (BTreeSet<String>, BTreeSet<String>) {
    let (mut system, mut publisher) = (BTreeSet::new(), BTreeSet::new());
    let userland = Path::new(env!("CARGO_MANIFEST_DIR")).join("..");
    for dir in fs::read_dir(userland).expect("userland") {
        let Ok(text) = fs::read_to_string(dir.expect("entry").path().join("Capsule.mk")) else {
            continue;
        };
        let ns = field(&text, "CAPSULE_NAMESPACE").next().expect("a namespace");
        let own = ns == "systems.nonos" || ns.starts_with("systems.nonos.");
        let eps =
            field(&text, "CAPSULE_SERVICE_ENDPOINT").chain(field(&text, "CAPSULE_REPLY_ENDPOINT"));
        for ep in eps {
            let name = ep.splitn(3, ':').nth(2).expect("kind:port:name").to_string();
            let set = if own { &mut system } else { &mut publisher };
            set.insert(name);
        }
    }
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
    assert!(publisher.iter().all(|n| !is_system_name(n)), "a publisher example collides");
}

#[test]
fn a_system_name_or_its_numbered_instance_is_refused_and_nothing_else() {
    for taken in ["input_router", "app.settings", "app.settings.1", "app.settings.12"] {
        assert!(is_system_name(taken), "{taken}");
    }
    for free in ["app.settings.", "app.settings.x", "app.settings.1a", "app.settingsx", "app"] {
        assert!(!is_system_name(free), "{free}");
    }
    assert!(!is_system_name("") && !is_system_name("com.example.app"));
}
