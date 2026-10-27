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

//! A program starts with its listed arguments before the person's own, and
//! the person's own option replaces the listed one of its name; and the
//! arguments leave with the program's box.

use super::market_args_tests::LISTED;
use crate::app::args_listed::with_listed;
use crate::install::market::minted::{beside, ARGS, ARGS_TRAILER, POLICY, POLICY_TRAILER, TRAILER};
use crate::install::market::take::take_out;

fn words(w: &[&str]) -> Vec<Vec<u8>> {
    w.iter().map(|w| w.as_bytes().to_vec()).collect()
}

#[test]
fn the_listed_words_come_first_unless_the_person_names_their_option() {
    assert_eq!(with_listed(LISTED, vec![]), words(&["--node=a:18", "--off"]));
    assert_eq!(with_listed(&[], words(&["x"])), words(&["x"]));
    let own = words(&["--node", "b:1", "w"]);
    assert_eq!(with_listed(LISTED, own.clone()), [words(&["--off"]), own].concat());
    let own = words(&["--node=b:1", "--off"]);
    assert_eq!(with_listed(LISTED, own.clone()), own, "each named one is the person's");
    let near = words(&["--nodes", "--of", "-off", "node"]);
    assert_eq!(
        with_listed(LISTED, near.clone()),
        [words(&["--node=a:18", "--off"]), near].concat()
    );
}

#[test]
fn after_a_double_dash_the_person_s_words_are_not_options() {
    let own = words(&["--", "--node=b:1"]);
    assert_eq!(with_listed(LISTED, own.clone()), [words(&["--node=a:18", "--off"]), own].concat());
}

#[test]
fn the_arguments_go_with_the_box() {
    let at = b"/usr/bin/monero-wallet-cli";
    let all = [TRAILER, POLICY, POLICY_TRAILER, ARGS, ARGS_TRAILER].map(|s| beside(at, s));
    let store = core::cell::RefCell::new([&[at.to_vec()][..], &all].concat());
    let mut unlink = |p: &[u8]| store.borrow_mut().retain(|k| k != p);
    let mut found = |p: &[u8]| Ok::<bool, ()>(store.borrow().iter().any(|k| k == p));
    assert!(take_out(at, &mut unlink, &mut found));
    assert!(store.borrow().is_empty(), "{:?}", store.borrow());
}
