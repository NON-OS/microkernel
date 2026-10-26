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

//! The development catalogue signed the way tools/nonos_seal/dev_market.py
//! signs it: every listing published by the operator key, each release
//! signed by `sign-release`, then the index by `sign`, all through the
//! binary the seal runs.

use std::fs;
use std::process::Command;

use serde_json::Value;

use crate::scratch::Scratch;

pub const SOURCE: &str = include_str!("../../../tools/devnet/market/index.json");

pub fn cli(args: &[&str]) -> String {
    let out =
        Command::new(env!("CARGO_BIN_EXE_marketplace-index")).args(args).output().expect("runs");
    assert!(out.status.success(), "{args:?}: {}", String::from_utf8_lossy(&out.stderr));
    String::from_utf8(out.stdout).expect("utf-8 output")
}

pub fn signed(dir: &Scratch, seed: &str, key: &[u8; 32]) -> Vec<u8> {
    let mut doc: Value = serde_json::from_str(SOURCE).expect("the development catalogue parses");
    let key = hex::encode(key);
    let mut releases = Vec::new();
    for entry in doc["entries"].as_array_mut().expect("entries") {
        assert_eq!(entry["publisher_pubkey"], "OPERATOR", "every listing is the copy operator's");
        entry["publisher_pubkey"] = Value::from(key.as_str());
        let listing = entry["listing_id"].as_str().expect("listing id").to_owned();
        for rel in entry["releases"].as_array().expect("releases") {
            releases.push((listing.clone(), rel["release_id"].as_str().expect("id").to_owned()));
        }
    }
    doc["serial"] = Value::from(1_791_466_416u64);
    let (json, blob) = (dir.path("index.json"), dir.path("index.bin"));
    fs::write(&json, doc.to_string()).expect("write the filled catalogue");
    for (listing, release) in &releases {
        let ids = ["--listing-id", listing, "--release-id", release];
        cli(&[&["sign-release", "--in", &json, "--key-file", seed, "--out", &json][..], &ids]
            .concat());
    }
    cli(&["sign", "--in", &json, "--key-file", seed, "--pubkey", &key, "--out", &blob]);
    fs::read(&blob).expect("the signed index")
}
