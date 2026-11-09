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

// Every build of a shipped binary drops the source path from its panic
// locations. Each entry is a file and a line in it that sets the
// flags for one build path; the line must carry location-detail=none. A new
// build path that sets RUSTFLAGS replaces the config's target flags, so it has
// to say it again, and this is where a missing one shows.

use std::fs;
use std::path::Path;

const FLAG: &str = "location-detail=none";

/// (file, a fragment naming the build line, how many such lines there are).
const BUILDS: &[(&str, &str, usize)] = &[
    // The kernel, both architectures, make and nix (no RUSTFLAGS there).
    (".cargo/config.toml", "\"-Z\", \"location-detail=none\"", 2),
    // The UEFI loader: its config, the make rule, the nix derivation.
    ("nonos-bootloader/.cargo/config.toml", "\"-Z\", \"location-detail=none\"", 1),
    ("mk/20-build.mk", "-C panic=abort -C target-feature=+crt-static", 1),
    ("tools/nix/image.nix", "-C panic=abort -C target-feature=+crt-static", 1),
    // Capsules and the upstream tools the image embeds.
    ("nonos-mk/capsule.mk", "_RUSTFLAGS        :=", 1),
    ("mk/20-build.mk", "RUSTFLAGS=\"-Zlocation-detail=none -Clink-arg=", 4),
    ("tools/nix/capsules.nix", "export RUSTFLAGS=", 2),
];

fn lines_with(root: &Path, file: &str, fragment: &str) -> Vec<String> {
    let text = fs::read_to_string(root.join(file)).unwrap_or_default();
    text.lines()
        .filter(|l| l.contains(fragment) && !l.trim_start().starts_with('#'))
        .map(str::to_owned)
        .collect()
}

#[test]
fn every_shipped_build_drops_panic_locations() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("..");
    for (file, fragment, count) in BUILDS {
        let found = lines_with(&root, file, fragment);
        assert_eq!(
            found.len(),
            *count,
            "{file}: expected {count} build lines naming {fragment:?}, found {found:?}"
        );
        for line in &found {
            assert!(line.contains(FLAG), "{file}: build line without {FLAG}: {line}");
        }
    }
}

#[test]
fn no_rustflags_line_in_the_build_rules_omits_it() {
    // Host tools clear RUSTFLAGS on purpose (RUSTFLAGS=""); every non-empty
    // setting in the make rules is a target build and must carry the flag.
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("..");
    for file in ["mk/20-build.mk", "tools/nix/capsules.nix", "tools/nix/image.nix"] {
        for line in lines_with(&root, file, "RUSTFLAGS=") {
            if line.contains("RUSTFLAGS=\"\"") {
                continue;
            }
            assert!(line.contains(FLAG), "{file}: {line}");
        }
    }
}
