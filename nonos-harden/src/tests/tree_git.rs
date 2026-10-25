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

//! The tree fixture's ledger and git: the ledger stamped as the seal stamps
//! it, and git run with a throwaway identity.

use super::tree_fixture::Tree;
use crate::hash::sha256_hex;
use crate::release::walk::files;
use crate::tree::ledger::TRUST;
use std::fs;
use std::process::Command;

impl Tree {
    /// Stamp the ledger over the trust set as it is now, as the seal does.
    pub fn ledger(&self) {
        let trust = self.path().join(TRUST);
        let mut lines = String::new();
        for part in ["capsules", "keys", "policy"] {
            for f in files(&trust.join(part), 1000).unwrap() {
                let rel = format!("{part}/{f}");
                lines += &format!("{}  {rel}\n", sha256_hex(&fs::read(trust.join(&rel)).unwrap()));
            }
        }
        fs::write(trust.join("MANIFEST.sha256"), lines).unwrap();
    }
    pub fn git(&self, args: &[&str]) -> String {
        let id = [
            "-c",
            "user.name=proof",
            "-c",
            "user.email=proof@nonos.invalid",
            "-c",
            "commit.gpgsign=false",
        ];
        let out =
            Command::new("git").arg("-C").arg(self.path()).args(id).args(args).output().unwrap();
        assert!(out.status.success(), "git {args:?}: {}", String::from_utf8_lossy(&out.stderr));
        String::from_utf8(out.stdout).unwrap()
    }
}
