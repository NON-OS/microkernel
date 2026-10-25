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

//! What any word the command line hands `--box` must leave: refused, or a
//! box whose every part is canonical and within what a box may grant, and
//! whose hash its own canonical spelling gives again. The fuzz target and
//! the seeded sweep both hold every word to this.

use super::folders::{folder, may_write};
use super::hash::box_hash;
use super::preset::{parse, BoxPolicy, Preset};
use nonos_linux_policy::dests::{check, entries, Kind};
use nonos_linux_policy::mask::{EXEC, FILES_HOME, NET_ANON, TTY};

pub fn check_word(word: &[u8]) -> Option<BoxPolicy> {
    let p = parse(word).ok()?;
    assert_eq!(p.preset == Preset::Learn, word == b"learn", "a learn run from another word");
    assert_eq!(p.mask & !(TTY | EXEC | FILES_HOME | NET_ANON), 0, "a box grants past its kinds");
    assert_eq!(p.mask & (TTY | EXEC), TTY | EXEC);
    assert!(p.folders.windows(2).all(|w| w[0] < w[1]), "folders unsorted or twice");
    for f in &p.folders {
        assert_eq!(folder(f).as_deref(), Ok(f.as_slice()), "a folder not canonical");
        assert!(may_write(&p.folders, f));
    }
    assert_eq!(
        p.dests.is_empty(),
        p.mask & NET_ANON == 0,
        "network without a list, or a list unused"
    );
    if !p.dests.is_empty() {
        assert!(check(&p.dests).is_ok(), "a list the personality refuses");
    }
    if p.preset == Preset::Allow {
        let again = parse(&spelled(&p)).expect("a box's own spelling is refused");
        assert_eq!(box_hash(&again), box_hash(&p), "a box hashes unlike its own spelling");
    }
    Some(p)
}

/// The `allow:` word that names exactly this box.
fn spelled(p: &BoxPolicy) -> Vec<u8> {
    let mut items: Vec<Vec<u8>> = p.folders.clone();
    for d in entries(&p.dests) {
        let host = match d.kind {
            Kind::V4 => {
                format!("{}.{}.{}.{}", d.body[0], d.body[1], d.body[2], d.body[3]).into_bytes()
            }
            _ => d.body.to_vec(),
        };
        items.push([b"net:".as_slice(), &host, format!(":{}", d.first_port).as_bytes()].concat());
    }
    [b"allow:".to_vec(), items.join(&b","[..])].concat()
}
