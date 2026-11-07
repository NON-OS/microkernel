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

//! A name the terminal's `linux` does not find in its own family's tree is
//! looked for along the PATH in the others' (terminal_launch/elsewhere.rs):
//! SimpleX's terminal client, installed in the Debian tree, is said to be
//! there and run by its package's word, not "command not found".

use alloc::string::String;
use alloc::vec;
use alloc::vec::Vec;

use crate::terminal_elsewhere::{found, line};

const TREES: [&[u8]; 2] = [b"/linux-deb", b"/linux-pacman"];

fn dirs() -> Vec<Vec<u8>> {
    [&b"/usr/local/bin"[..], b"/usr/bin", b"/bin"].iter().map(|d| d.to_vec()).collect()
}

/// What `found` asked the store, and its answer, for a store holding `held`.
fn ask(program: &[u8], dirs: &[Vec<u8>], held: &[&[u8]]) -> (Option<usize>, Vec<String>) {
    let mut asked = Vec::new();
    let got = found(program, dirs, &TREES, |tree, at| {
        let whole = [tree, at].concat();
        asked.push(String::from_utf8_lossy(&whole).into_owned());
        held.contains(&whole.as_slice())
    });
    (got, asked)
}

#[test]
fn a_debian_program_on_the_path_is_found_in_the_debian_tree() {
    let (got, _) = ask(b"simplex-chat", &dirs(), &[b"/linux-deb/usr/bin/simplex-chat"]);
    assert_eq!(got, Some(0));
    let (got, _) = ask(b"yay", &dirs(), &[b"/linux-pacman/bin/yay"]);
    assert_eq!(got, Some(1));
}

#[test]
fn a_name_no_tree_holds_is_not_found_after_every_place_is_asked() {
    let (got, asked) = ask(b"nothing", &dirs(), &[b"/linux-deb/usr/bin/other"]);
    assert_eq!(got, None);
    assert_eq!(asked.len(), 6, "each tree along each PATH directory");
    assert!(asked.iter().all(|a| a.ends_with("/nothing")));
}

#[test]
fn only_a_plain_name_is_looked_for() {
    for name in [&b""[..], b".", b"..", b"bin/sh", b"/usr/bin/simplex-chat", b"../linux/bin/sh"] {
        let (got, asked) = ask(name, &dirs(), &[b"/linux-deb/usr/bin/simplex-chat"]);
        assert_eq!((got, asked.len()), (None, 0), "{:?}", String::from_utf8_lossy(name));
    }
}

#[test]
fn every_path_asked_stays_under_its_tree() {
    let odd = vec![
        b"/usr/../..".to_vec(),
        b"relative/bin".to_vec(),
        b"/usr/./bin".to_vec(),
        b"/".to_vec(),
        b"/sbin/".to_vec(),
    ];
    let (_, asked) = ask(b"x", &odd, &[]);
    assert_eq!(
        asked,
        ["/linux-deb/x", "/linux-deb/sbin/x", "/linux-pacman/x", "/linux-pacman/sbin/x"]
    );
}

#[test]
fn the_line_names_the_program_the_tree_and_the_word_it_runs_by() {
    let said = line("simplex-chat", "Debian", "kali.");
    assert!(said.starts_with("linux: simplex-chat is installed in the Debian family's tree"));
    assert!(said.contains("`linux kali.<package>`"));
    assert!(said.contains("open it from the Marketplace"));
    assert!(said.ends_with('\n'));
}
