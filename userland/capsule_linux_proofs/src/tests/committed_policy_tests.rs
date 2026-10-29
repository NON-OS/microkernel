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

//! SERVE_ONE's P.2, against the market catalogue: version 2 moves no version
//! 1 byte, so every committed guest policy encodes to itself and keeps its
//! hash, and so does it with a field its SERVE_ONE bit does not grant. Here,
//! not in nonos_linux_policy, so a catalogue edit rebuilds no capsule. The one
//! version 2 policy, the SimpleX core's, is the pack tool's bytes decoded here.

use nonos_linux_policy::{decode, market_ready, policy_hash, LinuxAppPolicy, ServeOne, LEN};

/// The market catalogue the operator publishes, each release's guest_policy.
const CATALOGUE: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../../nonos-data/market/catalogue");
const KEY: &str = "\"guest_policy\": \"";

#[test]
fn every_committed_policy_keeps_its_bytes_and_hash() {
    let serve = ServeOne::new(5225, b"app.simplex");
    let (mut seen, mut served) = (0, Vec::new());
    for file in std::fs::read_dir(CATALOGUE).unwrap() {
        let path = file.unwrap().path();
        let text = std::fs::read_to_string(&path).unwrap();
        for hex in text.split(KEY).skip(1) {
            let b: [u8; LEN] =
                core::array::from_fn(|i| u8::from_str_radix(&hex[2 * i..2 * i + 2], 16).unwrap());
            let p = decode(&b).unwrap();
            assert_eq!(p.encode(), b);
            seen += 1;
            if let Some(s) = p.serve_one() {
                let doc = path.file_name().unwrap().to_string_lossy().into_owned();
                served.push((doc, s.port, s.name().to_owned(), b[8], market_ready(&p)));
                continue;
            }
            let unserved = LinuxAppPolicy { serve, ..p };
            assert_eq!(unserved.serve_one(), None);
            assert_eq!(policy_hash(&unserved.encode()), policy_hash(&b));
        }
    }
    assert_ne!(seen, 0, "no guest_policy under {CATALOGUE}");
    let core = "linux.kali.nonos-simplex-core.json".to_owned();
    assert_eq!(served, [(core, 5225, "app.simplex".to_owned(), 2, true)]);
}
