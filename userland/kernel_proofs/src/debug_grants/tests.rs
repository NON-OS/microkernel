// NØNOS Operating System
// Copyright (C) 2026 NØNOS Contributors
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

use super::drivers::DRIVERS;

/// The spawn's requested_caps expression, up to the field after it.
fn requested(spawn: &str) -> &str {
    let at = spawn.find("requested_caps:").expect("no requested_caps");
    let end = spawn[at..].find("debug_tag:").map_or(spawn.len(), |e| at + e);
    &spawn[at..end]
}

#[test]
fn each_spawn_folds_in_the_optional_debug() {
    for (name, spawn, _) in DRIVERS {
        assert!(
            requested(spawn).contains("| crate::capabilities::serial_debug_cap()"),
            "{name}'s spawn asks no Debug, so a development image's console lacks its lines"
        );
    }
}

#[test]
fn each_manifest_declares_debug_optional_and_nothing_else() {
    for (name, _, mk) in DRIVERS {
        let line = mk.lines().find(|l| l.starts_with("CAPSULE_OPTIONAL_CAPS"));
        let value = line.and_then(|l| l.split(":=").nth(1)).map(str::trim);
        assert_eq!(value, Some("0x100"), "{name}'s manifest");
    }
}

#[test]
fn no_spawn_asks_debug_outright() {
    for (name, spawn, _) in DRIVERS {
        assert!(
            !requested(spawn).contains("Capability::Debug"),
            "{name} asks Debug in every build"
        );
    }
}
