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

//! A tree for a proof: a git repository with a trust set, its ledger and the
//! kernel's public keys (written by nonos-sign, as the seal writes them), the
//! markers `tree::find` looks for, and the private halves kept here to sign
//! with. Nothing in it is a real key.

use crate::release::kernel_keys::{ED_PUB, PQ_PUB};
use crate::tree::ledger::TRUST;
use ed25519_dalek::SigningKey;
use nonos_capsule_sign::algs::{mldsa65::MlDsa65, AlgId, Signer};
use nonos_capsule_sign::keys::write_pub;
use std::fs;
use std::path::Path;
use tempfile::TempDir;

pub struct Tree {
    pub dir: TempDir,
    pub ed: SigningKey,
    pub pq_pub: Vec<u8>,
    pub pq_secret: Vec<u8>,
    pub commit: String,
}

impl Tree {
    pub fn new() -> Self {
        let pq = MlDsa65::keygen_random().unwrap();
        let (dir, ed) = (tempfile::tempdir().unwrap(), SigningKey::from_bytes(&[7; 32]));
        let mut t = Self {
            dir,
            ed,
            pq_pub: pq.pubkey.clone(),
            pq_secret: pq.seed.clone(),
            commit: String::new(),
        };
        t.put("nonos-verify/Cargo.toml", b"[package]\nname = \"stand-in\"\n");
        t.put("tools/nonos-verify-matrix", b"stand-in\n");
        t.put("src/main.rs", b"fn main() {}\n");
        t.put(crate::tree::declared::FILE, super::fake_report::DECLARED);
        t.put(&format!("{TRUST}/capsules/about.manifest.bin"), b"a capsule manifest");
        t.put(&format!("{TRUST}/policy/kernel_attest_root.bin"), b"a root");
        fs::create_dir_all(t.path().join(TRUST).join("keys")).unwrap();
        // Raw, as the seal writes it; the ML-DSA-65 key in a NONOSPK1 file.
        fs::write(t.path().join(ED_PUB), t.ed.verifying_key().as_bytes()).unwrap();
        write_pub(&t.path().join(PQ_PUB), AlgId::MlDsa65, &t.pq_pub).unwrap();
        t.ledger();
        t.git(&["init", "-q"]);
        t.git(&["add", "-A"]);
        t.git(&["commit", "-q", "-m", "the release's source"]);
        t.commit = t.git(&["rev-parse", "HEAD"]).trim().to_string();
        t
    }
    pub fn path(&self) -> &Path {
        self.dir.path()
    }
    pub fn put(&self, rel: &str, bytes: &[u8]) {
        let p = self.path().join(rel);
        fs::create_dir_all(p.parent().unwrap()).unwrap();
        fs::write(p, bytes).unwrap();
    }
}
