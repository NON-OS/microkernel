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

//! The seeded sweep of `allow:` words: built from sound items so that most
//! are taken, each held to `check_word`, so a canonical box is tested, not
//! only a refused one.

use super::super::random::Regs;
use crate::boxed::check::check_word;

/// An `allow:` word of one to eight items, most of them sound, any one
/// perhaps spoiled by a byte.
fn allow_word(r: &mut Regs) -> Vec<u8> {
    const OWN: &[&str] = &["/tmp", "/home", "/root", "/run", "/var/tmp", "/dev/shm", "/etc"];
    const NAMES: &[&str] = &["example.com", "a.b", "x-y.org", "Mixed.Case", "1.2.3", "-bad.com"];
    let mut items = Vec::new();
    for _ in 0..1 + r.any() % 8 {
        let item = match r.any() % 3 {
            0 => format!("{}/{}", OWN[(r.any() as usize) % OWN.len()], r.any() % 4),
            1 => format!(
                "net:{}.{}.{}.{}:{}",
                r.any() % 300,
                r.any() % 256,
                r.any() % 256,
                r.any() % 256,
                r.any() % 70_000
            ),
            _ => format!("net:{}:{}", NAMES[(r.any() as usize) % NAMES.len()], r.any() % 70_000),
        };
        items.push(item.into_bytes());
    }
    if r.any().is_multiple_of(4) {
        let i = (r.any() as usize) % items.len();
        let at = (r.any() as usize) % (items[i].len() + 1);
        items[i].insert(at, r.any() as u8);
    }
    [b"allow:".to_vec(), items.join(&b","[..])].concat()
}

#[test]
fn every_allow_word_is_refused_or_a_canonical_box() {
    let mut r = Regs::new(0xa110_5eed);
    let taken = (0..100_000).filter(|_| check_word(&allow_word(&mut r)).is_some()).count();
    assert!(taken > 10_000, "only {taken} allow boxes taken: the sweep tests refusals alone");
}
