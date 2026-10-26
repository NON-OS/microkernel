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

//! Part B over the whole kernel (P1, P5 and the rest marks): only send
//! writes a run to the device, only write_deferred holds, only the sealed
//! tree writes deferred, and every take of PENDING is marked quiet.

use std::path::Path;

use crate::proofs::proofs_8::transport_takeover::text::code;

/// Every kernel source file, as (path under src/, code alone).
fn sources() -> Vec<(String, String)> {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../src");
    let (mut todo, mut out) = (vec![root.clone()], Vec::new());
    while let Some(dir) = todo.pop() {
        for e in std::fs::read_dir(&dir).expect("src/ reads").map(|e| e.expect("entry")) {
            let p = e.path();
            if p.is_dir() {
                todo.push(p);
            } else if p.extension().is_some_and(|x| x == "rs") {
                let name = p.strip_prefix(&root).expect("under src").display().to_string();
                out.push((name, code(&std::fs::read_to_string(&p).expect("reads"))));
            }
        }
    }
    assert!(out.len() > 1000, "the walk found the kernel: {}", out.len());
    out
}

fn callers(all: &[(String, String)], needle: &str) -> Vec<String> {
    let mut v: Vec<String> =
        all.iter().filter(|(_, c)| c.contains(&code(needle))).map(|(n, _)| n.clone()).collect();
    v.sort();
    v
}

#[test]
fn p1_p5_who_sends_who_holds_and_who_writes_deferred() {
    let all = sources();
    assert_eq!(callers(&all, "super::device::write("), ["fs/cryptoblock/pending_send.rs"]);
    assert_eq!(callers(&all, "pending::hold("), ["fs/cryptoblock/write_deferred.rs"]);
    let deferred = callers(&all, "cryptoblock::write_deferred(");
    assert_eq!(deferred, ["fs/blockfs/tree_sealed.rs"]);
}

#[test]
fn every_take_of_pending_is_marked_quiet_in_its_block() {
    // Other modules name their own locks PENDING; this one is the cryptoblock's.
    let mine = |(n, _): &(String, String)| n.starts_with("fs/cryptoblock/");
    let all: Vec<_> = sources().into_iter().filter(mine).collect();
    assert!(callers(&all, "PENDING.lock(").is_empty(), "PENDING is taken one way only");
    let takers = callers(&all, "lock_responsive(&PENDING)");
    assert_eq!(takers, ["fs/cryptoblock/pending.rs", "fs/cryptoblock/pending_send.rs"]);
    for (name, c) in all.iter().filter(|(n, _)| takers.contains(n)) {
        for (at, _) in c.match_indices("lock_responsive(&PENDING)") {
            let block = &c[c[..at].rfind('{').expect("in a block")..at];
            assert!(block.contains("quiet()"), "{name}: a take of PENDING with no quiet mark");
        }
    }
}
