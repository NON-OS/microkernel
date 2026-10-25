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

//! Which trust set the kernel build embeds.

use std::env;
use std::path::PathBuf;

// The trust set is nonos-data/trust, or the directory NONOS_BAKED_TRUST_DIR
// names: a test set signed with keys made outside the tree, so a test build
// never writes the tree's. The kernel reads it as NONOS_BAKED_TRUST through
// baked_trust! (src/security/nonos_trust_anchor/baked_trust.rs).
pub fn declare_baked_trust() -> PathBuf {
    println!("cargo:rerun-if-env-changed=NONOS_BAKED_TRUST_DIR");
    let release = env::var_os("CARGO_FEATURE_NONOS_RELEASE").is_some();
    let named = TEST_TRUST.map(|v| (v, env::var_os(v).is_some()));
    if let Some(why) = refusal(release, &named) {
        panic!("{why}");
    }
    let manifest = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let dir = match env::var_os("NONOS_BAKED_TRUST_DIR") {
        Some(d) => manifest.join(d),
        // Named by its policy directory, the one every trust set holds.
        None => manifest.join("nonos-data/trust/policy").parent().unwrap().to_path_buf(),
    };
    if env::var_os("NONOS_BAKED_TRUST_DIR").is_some() && !dir.join("policy").is_dir() {
        panic!("NONOS_BAKED_TRUST_DIR names {}, which holds no policy/", dir.display());
    }
    for sub in ["capsules", "keys", "policy"] {
        println!("cargo:rerun-if-changed={}", dir.join(sub).display());
    }
    println!("cargo:rustc-env=NONOS_BAKED_TRUST={}", dir.display());
    dir
}

/// What may stand in for the release's trust: a test set and a test policy key.
pub const TEST_TRUST: [&str; 2] = ["NONOS_BAKED_TRUST_DIR", "NONOS_DEVICE_POLICY_PUBKEY"];

/// Why a build may not go on: a kernel the seal may seal for release
/// (nonos-release) embeds only the tree's trust, never a set or key named at
/// build time.
pub fn refusal(release: bool, named: &[(&str, bool)]) -> Option<String> {
    let set: Vec<&str> = named.iter().filter(|(_, on)| *on).map(|(v, _)| *v).collect();
    (release && !set.is_empty()).then(|| {
        format!("a nonos-release kernel embeds only the tree's trust; unset {}", set.join(", "))
    })
}
