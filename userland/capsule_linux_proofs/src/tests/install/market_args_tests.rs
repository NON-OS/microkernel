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

//! A release's listed arguments on the install word: carried after the 640
//! hex, bounded, read whole before anything is fetched, and taken out with
//! the program's box.

use super::super::market_tests::{market, policy, ready};
use crate::install::market::arg::{market_arg, Market, NotMarket, MARKET_LEN};
use crate::install::market::check::gate;
use nonos_linux_policy::args::MAX_LEN;

pub(super) const LISTED: &[u8] = b"NLXARG01\x02\x0b--node=a:18\x05--off";

fn hex(b: &[u8]) -> String {
    b.iter().map(|b| format!("{b:02x}")).collect()
}

#[test]
fn the_arguments_ride_after_the_keys_up_to_their_bound() {
    let m = market(&policy(1, 2, ready()), 1, 2);
    let word = hex(&[&m[..], LISTED].concat());
    let read = market_arg(word.as_bytes()).expect("read").expect("a market word");
    assert_eq!(read, Market { head: m, args: LISTED.to_vec() });
    let most = hex(&[&m[..], &[7; MAX_LEN]].concat());
    assert!(matches!(market_arg(most.as_bytes()), Ok(Some(x)) if x.args.len() == MAX_LEN));
    let past = hex(&[&m[..], &[7; MAX_LEN + 1]].concat());
    for bad in [
        past.clone(),
        format!("{word}0"),
        word[..MARKET_LEN * 2 - 2].to_string(),
        word.to_uppercase(),
    ] {
        assert_eq!(market_arg(bad.as_bytes()), Err(NotMarket), "{} hex", bad.len());
    }
}

#[test]
fn arguments_that_do_not_read_whole_fail_the_install_before_any_fetch() {
    let p = policy(1, 2, ready());
    let sold = |args: &[u8]| Market { head: market(&p, 1, 2), args: args.to_vec() };
    assert_eq!(gate("git", Some(&sold(LISTED))), Ok(Some(p)));
    let malformed = Err("its listed arguments are malformed");
    for bad in [&b"NLXARG01"[..], b"NLXARG01\x01\x03-ab", &LISTED[..LISTED.len() - 1], &[7; 40]] {
        assert_eq!(gate("git", Some(&sold(bad))), malformed, "{bad:?}");
    }
    let mut twice = LISTED.to_vec();
    twice.splice(9..21, *b"\x05--off");
    assert_eq!(gate("git", Some(&sold(&twice))), malformed, "an option twice");
}
