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

use crate::hash::sha256_hex;
use serde_json::{json, Map, Value};
use std::fs;
use std::path::Path;
use tempfile::TempDir;

pub const ESP: [&str; 2] = ["EFI/BOOT/BOOTX64.EFI", "kernel.bin"];

pub struct Release {
    pub dir: TempDir,
}

impl Release {
    pub fn dev() -> Self {
        Self::with(json!({"release": false, "profile": "qemu-dev", "features": ["nonos-dev-attest"]}))
    }
    pub fn release() -> Self {
        Self::with(json!({"release": true, "profile": "qemu", "features": [], "rollback_index": 1}))
    }
    pub fn with(config: Value) -> Self {
        let r = Self { dir: tempfile::tempdir().unwrap() };
        let image: Vec<u8> = (0..4096u32).map(|i| (i * 7 % 251) as u8).collect();
        r.put("nonos.img", &image);
        r.put("kernel.bin", b"the signed kernel");
        r.put("esp/EFI/BOOT/BOOTX64.EFI", b"the loader");
        r.put("esp/kernel.bin", b"the signed kernel");
        r.record(config);
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
    fn sha(&self, rel: &str) -> String {
        sha256_hex(&fs::read(self.path().join(rel)).unwrap())
    }
    /// The record over what the directory holds now, every digest real.
    pub fn record(&self, config: Value) {
        let file = |rel: &str| json!({"file": rel, "sha256": self.sha(rel)});
        let esp: Map<String, Value> = ESP.iter().map(|f| (f.to_string(), json!(self.sha(&format!("esp/{f}"))))).collect();
        let sealed = json!({"usb": file("nonos.img"), "kernel": file("kernel.bin"), "esp": {"dir": "esp", "files": esp}});
        let rec = json!({"format": "nonos-release/1", "config": config, "build": {}, "sealed": sealed});
        fs::write(self.path().join("nonos-release.json"), serde_json::to_vec_pretty(&rec).unwrap()).unwrap();
    }
    /// The middle byte of `rel` inverted, as tools/nonos-flip-byte does.
    pub fn flip(&self, rel: &str) {
        let p = self.path().join(rel);
        let mut b = fs::read(&p).unwrap();
        let at = b.len() / 2;
        b[at] ^= 0xFF;
        fs::write(p, b).unwrap();
    }
}
