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

//! A release sealed from a tree, as the seal leaves the two: the image, the
//! signed and attested kernel (in the ESP too), the loader, the record of
//! every digest, and the build receipt naming the tree's commit.

use super::attest::{attested, KERNEL, ROLLBACK};
use super::fake::Fake;
use super::fixture::{dev_config, release_config, Release};
use super::tree_fixture::Tree;
use crate::ctx::Ctx;
use serde_json::{json, Value};
use std::fs;

pub struct Sealed {
    pub tree: Tree,
    pub release: Release,
}

impl Sealed {
    pub fn dev() -> Self {
        Self::new(dev_config())
    }
    pub fn release() -> Self {
        Self::new(release_config())
    }
    pub fn new(config: Value) -> Self {
        let (tree, release) = (Tree::new(), Release::with(config));
        let kernel = attested(&tree, KERNEL, ROLLBACK);
        release.put("kernel.bin", &kernel);
        release.put("esp/kernel.bin", &kernel);
        release.record();
        let s = Self { tree, release };
        s.receipt(false);
        s
    }
    /// The build receipt the seal writes (scripts/build_receipt.py's shape).
    pub fn receipt(&self, dirty: bool) {
        let art = |rel: &str| json!({"bytes": 1, "sha256": self.release.sha(rel)});
        let roots = json!({"capsule_policy": {"root": "r", "embedded_in_kernel": true},
            "kernel_attest": {"root": "r", "embedded_in": "kernel", "ok": true}});
        let artifacts = json!({"/seal/work/kernel.bin": art("kernel.bin"), "/seal/out/nonos.img": art("nonos.img")});
        let r = json!({"schema": "nonos.build.receipt.v1", "roots": roots, "artifacts": artifacts,
            "source": {"commit": self.tree.commit, "branch": "main", "dirty": dirty},
            "verification": {"root_embedding": "pass"}});
        fs::write(self.release.path().join("build-receipt.json"), r.to_string()).unwrap();
    }
    pub fn ctx(&self, fake: Fake) -> Ctx {
        Ctx::with_runner(self.release.path().into(), Some(self.tree.path().into()), Box::new(fake))
    }
}
