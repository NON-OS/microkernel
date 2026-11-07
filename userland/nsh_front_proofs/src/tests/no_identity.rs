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

//! nsh.front holds no host secret and compiles no identity in: no
//! `include_bytes!`, no secret key type, no path into the identity files
//! the devkit once wrote. Its signatures come from nsh.attest, and the
//! machine's records are read for each connection. nsh.attest writes the
//! host keys only sealed.

use std::fs;
use std::path::Path;

fn sources(dir: &str) -> Vec<(String, String)> {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("..").join(dir);
    let mut out = Vec::new();
    let mut todo = vec![root];
    while let Some(d) = todo.pop() {
        for e in fs::read_dir(&d).expect("a source dir") {
            let p = e.expect("an entry").path();
            if p.is_dir() {
                todo.push(p);
            } else if p.extension().is_some_and(|x| x == "rs") {
                out.push((p.display().to_string(), fs::read_to_string(&p).expect("source")));
            }
        }
    }
    assert!(!out.is_empty());
    out
}

#[test]
fn nsh_front_compiles_in_no_identity_and_holds_no_secret_key() {
    for (path, src) in sources("capsule_nsh_front/src") {
        for banned in ["include_bytes!", "identity/", "SecretKey", "HostKey::", "Secret::take"] {
            assert!(!src.contains(banned), "{path} holds {banned}");
        }
    }
    let manifest = include_str!("../../../capsule_nsh_front/Cargo.toml");
    assert!(!manifest.contains("nonos-mldsa65"), "no ML-DSA secret key type");
    assert!(!manifest.contains("nsh_crypto"));
    assert!(
        !Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/../capsule_nsh_front/identity")).exists()
    );
}

#[test]
fn nsh_front_signs_through_nsh_attest_and_reads_records_per_connection() {
    let session = include_str!("../../../capsule_nsh_front/src/session.rs");
    let start = &session[session.find("fn start(").expect("start")..];
    let fetch = start.find("HostBridge::fetch(attest_port)").expect("the keys from nsh.attest");
    let records = start.find("records::fetch(attest_port)?").expect("the records");
    let server =
        start.find("Server::new(cfg, kernel, signer, roles, clock::now())").expect("server");
    assert!(fetch < records && records < server);
    assert!(session.contains("pub type Host = Server<AttestBridge, HostBridge, RoleStore>;"));
    let host = include_str!("../../../capsule_nsh_front/src/host.rs");
    assert!(host.contains("relay(self.port, OP_SIGN, digest, &mut resp)"));
}

#[test]
fn nsh_attest_writes_the_host_keys_only_sealed() {
    let keys = include_str!("../../../capsule_nsh_attest/src/keys/mod.rs");
    let puts: Vec<_> =
        keys.match_indices("store::put(KEYS_PATH, ").map(|(i, _)| &keys[i..i + 40]).collect();
    assert_eq!(puts.len(), 2, "made, and kept later");
    assert!(puts.iter().all(|p| p.starts_with("store::put(KEYS_PATH, &sealed[..])")), "{puts:?}");
    assert!(
        !keys.contains("store::put(KEYS_PATH, plain")
            && !keys.contains("store::put(KEYS_PATH, &plain")
    );
}

fn caps(mk: &str, key: &str) -> u64 {
    let line = mk.lines().find(|l| l.starts_with(key)).unwrap_or_else(|| panic!("{key}"));
    let hex = line.rsplit("0x").next().expect("a mask").trim();
    u64::from_str_radix(hex, 16).expect("hex")
}

#[test]
fn the_split_holds_the_keys_and_the_quote_away_from_the_network() {
    const NETWORK: u64 = 1 << 2;
    const CRYPTO: u64 = 1 << 5;
    const FILE_SYSTEM: u64 = 1 << 6;
    const ATTEST_READ: u64 = 1 << 31;
    let attest = include_str!("../../../capsule_nsh_attest/Capsule.mk");
    let front = include_str!("../../../capsule_nsh_front/Capsule.mk");
    let a = caps(attest, "CAPSULE_REQUIRED_CAPS") | caps(attest, "CAPSULE_OPTIONAL_CAPS");
    let f = caps(front, "CAPSULE_REQUIRED_CAPS") | caps(front, "CAPSULE_OPTIONAL_CAPS");
    assert_eq!(a & NETWORK, 0, "nsh.attest never reaches the network");
    assert_eq!(a & (ATTEST_READ | CRYPTO | FILE_SYSTEM), ATTEST_READ | CRYPTO | FILE_SYSTEM);
    assert_eq!(
        f & (ATTEST_READ | CRYPTO | FILE_SYSTEM),
        0,
        "nsh.front holds no key, quote or file"
    );
    assert_eq!(f & NETWORK, NETWORK);
}
