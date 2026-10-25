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

//! No function in net.nym hands the gateway key out by value: every reader
//! borrows it under the slot's lock. Read from the whole capsule's source.

const ROOT: &str = "../capsule_net_nym/src/";

#[test]
fn no_getter_hands_the_key_out_by_value() {
    let mut copied = Vec::new();
    for entry in walk(std::path::Path::new(ROOT)) {
        let text = std::fs::read_to_string(&entry).unwrap_or_default();
        if text.contains("fn gateway_shared_key(")
            || text.contains("-> Option<[u8; 32]> {\n    let slot = SHARED_KEY")
        {
            copied.push(entry);
        }
    }
    assert!(copied.is_empty(), "a getter that hands the key out by value: {copied:?}");
}

fn walk(dir: &std::path::Path) -> Vec<std::path::PathBuf> {
    let mut out = Vec::new();
    for e in std::fs::read_dir(dir).expect("the capsule's source").flatten() {
        let p = e.path();
        if p.is_dir() {
            out.extend(walk(&p));
        } else if p.extension().is_some_and(|x| x == "rs") {
            out.push(p);
        }
    }
    out
}
