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

//! A release's extra packages on the install word: after its listed
//! arguments or alone, read whole before anything is fetched, at most
//! with::MAX_NAMES of them, each by the package rule.

use super::super::market_tests::{market, policy, ready};
use super::market_args_tests::LISTED;
use crate::install::market::arg::{market_arg, Market};
use crate::install::market::check::gate;
use nonos_linux_policy::with::MAX_NAMES;

const WITH: &[u8] = b"NLXWITH1\x02\x11zathura-pdf-mupdf\x0cpoppler-data";

fn block(names: &[&str]) -> Vec<u8> {
    let mut b = b"NLXWITH1".to_vec();
    b.push(names.len() as u8);
    names.iter().for_each(|n| b.extend([&[n.len() as u8][..], n.as_bytes()].concat()));
    b
}

#[test]
fn the_extras_ride_after_the_arguments_or_alone() {
    let p = policy(1, 2, ready());
    let m = market(&p, 1, 2);
    for (tail, args) in [([LISTED, WITH].concat(), LISTED), (WITH.to_vec(), &b""[..])] {
        let word: String = [&m[..], &tail].concat().iter().map(|b| format!("{b:02x}")).collect();
        let read = market_arg(word.as_bytes()).expect("read").expect("a market word");
        assert_eq!(gate("zathura", Some(&read)), Ok(Some(p)));
        assert_eq!(read.tail(), Ok((args, vec!["zathura-pdf-mupdf", "poppler-data"])));
    }
    let none = Market { head: m, args: LISTED.to_vec() };
    assert_eq!(none.tail(), Ok((LISTED, Vec::new())));
}

#[test]
fn extras_that_do_not_read_whole_fail_the_install_before_any_fetch() {
    let p = policy(1, 2, ready());
    let sold = |tail: &[u8]| Market { head: market(&p, 1, 2), args: tail.to_vec() };
    let nine: Vec<String> = (0..=MAX_NAMES).map(|i| format!("p{i}")).collect();
    let nine: Vec<&str> = nine.iter().map(String::as_str).collect();
    assert!(gate("zathura", Some(&sold(&block(&nine[..MAX_NAMES])))).is_ok());
    let malformed = Err("its extra packages are malformed");
    for bad in [
        block(&["A"]),
        block(&["../x"]),
        block(&["-x"]),
        block(&[&"a".repeat(65)]),
        block(&["a", "a"]),
        block(&nine),
        block(&[]),
        WITH[..WITH.len() - 1].to_vec(),
        [WITH, b"x"].concat(),
        [WITH, LISTED].concat(),
        [LISTED, b"junk"].concat(),
    ] {
        assert_eq!(gate("zathura", Some(&sold(&bad))), malformed, "{bad:?}");
    }
}
