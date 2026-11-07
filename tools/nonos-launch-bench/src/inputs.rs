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

//! The committed capsules: each one's NONOS ID certificate and manifest, and
//! the trust anchor policy the kernel bakes in.

use std::fs;
use std::path::Path;

pub struct Capsule {
    pub name: String,
    pub cert: Vec<u8>,
    pub manifest: Vec<u8>,
    pub trailer: Vec<u8>,
}

pub struct Inputs {
    pub policy: Vec<u8>,
    pub capsules: Vec<Capsule>,
}

pub fn load(root: &Path) -> Result<Inputs, String> {
    let trust = root.join("nonos-data/trust");
    let policy_path = trust.join("policy/nonos_trust_anchor.policy.bin");
    let policy = fs::read(&policy_path).map_err(|e| format!("{}: {e}", policy_path.display()))?;
    let dir = trust.join("capsules");
    let mut names: Vec<String> = fs::read_dir(&dir)
        .map_err(|e| format!("{}: {e}", dir.display()))?
        .filter_map(|e| e.ok())
        .filter_map(|e| e.file_name().into_string().ok())
        .filter_map(|n| n.strip_suffix(".manifest.bin").map(str::to_string))
        .collect();
    names.sort();
    let mut capsules = Vec::new();
    for name in names {
        let read = |suffix: &str| {
            let p = dir.join(format!("{name}{suffix}"));
            fs::read(&p).map_err(|e| format!("{}: {e}", p.display()))
        };
        capsules.push(Capsule {
            cert: read(".nonos_id_cert.bin")?,
            manifest: read(".manifest.bin")?,
            trailer: read(".zk_trailer.bin")?,
            name,
        });
    }
    if capsules.is_empty() {
        return Err(format!("no capsules under {}", dir.display()));
    }
    Ok(Inputs { policy, capsules })
}
