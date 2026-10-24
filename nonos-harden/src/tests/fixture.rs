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

//! A sealed release for a proof: real files and a record of their real digests.

pub use super::configs::{dev_config, release_config};
use crate::hash::sha256_hex;
use serde_json::{json, Map, Value};
use std::fs;
use std::path::Path;
use tempfile::TempDir;

pub const ESP: [&str; 2] = ["EFI/BOOT/BOOTX64.EFI", "kernel.bin"];

pub struct Release {
    pub dir: TempDir,
    pub config: Value,
}

impl Release {
    pub fn dev() -> Self {
        Self::with(dev_config())
    }
    pub fn with(config: Value) -> Self {
        let r = Self { dir: tempfile::tempdir().unwrap(), config };
        r.put("nonos.img", &(0..4096u32).map(|i| (i * 7 % 251) as u8).collect::<Vec<_>>());
        r.put("kernel.bin", b"the signed kernel");
        r.put("esp/kernel.bin", b"the signed kernel");
        r.put("esp/EFI/BOOT/BOOTX64.EFI", b"the loader");
        r.record();
        r
    }
    pub fn path(&self) -> &Path {
        self.dir.path()
    }
    pub fn put(&self, rel: &str, bytes: &[u8]) {
        let p = self.path().join(rel);
        fs::create_dir_all(p.parent().unwrap()).unwrap();
        fs::write(p, bytes).unwrap();
    }
    pub fn sha(&self, rel: &str) -> String {
        sha256_hex(&fs::read(self.path().join(rel)).unwrap())
    }
    /// The record over what the directory holds now, every digest real.
    pub fn record(&self) {
        let file = |rel: &str| json!({"file": rel, "sha256": self.sha(rel)});
        let esp: Map<String, Value> =
            ESP.iter().map(|f| (f.to_string(), json!(self.sha(&format!("esp/{f}"))))).collect();
        let sealed = json!({"usb": file("nonos.img"), "kernel": file("kernel.bin"), "esp": {"dir": "esp", "files": esp}});
        let rec = json!({"format": "nonos-release/1", "config": self.config, "build": {}, "sealed": sealed});
        fs::write(self.path().join("nonos-release.json"), rec.to_string()).unwrap();
    }
}
