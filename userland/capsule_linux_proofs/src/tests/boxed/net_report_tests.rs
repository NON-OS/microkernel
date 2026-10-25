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

//! A box naming destinations reports each one it allowed and each one the run
//! connected to, on its report and on one `[BOX-NET]` line before `[BOX]`; a
//! box with none adds nothing, so its report and log are as they were.

use crate::boxed::connected::Connected;
use crate::boxed::hash::box_hash;
use crate::boxed::net_report::{connected, net_line};
use crate::boxed::preset::parse;
use crate::boxed::report::{report, End};
use crate::boxed::tally::Tally;
use nonos_linux_policy::mask::{EXEC, FILES_HOME, NET_ANON, TTY};

const NET: &[u8] = b"allow:/tmp/o,net:10.0.2.2:80,net:Api.Example.com:443";

pub fn reached(dests: &[&[u8]]) -> Connected {
    let mut c = Connected::default();
    dests.iter().for_each(|d| c.note(d));
    c
}

#[test]
fn the_box_net_line_lists_allowed_and_connected() {
    let p = parse(NET).unwrap();
    let c = reached(&[b"api.example.com:443"]);
    let want = "[BOX-NET] v=1 allowed=10.0.2.2:80,api.example.com:443 \
                connected=api.example.com:443\n";
    assert_eq!(net_line(&p, c.all()).unwrap(), want);
    let none = "[BOX-NET] v=1 allowed=10.0.2.2:80,api.example.com:443 connected=none\n";
    assert_eq!(net_line(&p, &[]).unwrap(), none);
    assert_eq!(connected(&p, &[]).unwrap(), "connected  none");
}

#[test]
fn the_report_says_connected_after_what_it_allowed() {
    let p = parse(NET).unwrap();
    let c = reached(&[b"10.0.2.2:80", b"api.example.com:443"]);
    let e = End { exit: 0, not_started: None, reached: c.all() };
    let mask = Some(TTY | EXEC | FILES_HOME | NET_ANON);
    let text = report(&p, &box_hash(&p), &Tally::default(), mask, &e);
    let want = "  allowed  network 10.0.2.2:80 api.example.com:443\n  \
                allowed  execve, each program held to this box\n  \
                connected  10.0.2.2:80 api.example.com:443\n  refused  0\n";
    assert!(text.contains(want), "{text}");
}

#[test]
fn a_box_with_no_destination_adds_nothing() {
    for word in [&b"strict"[..], b"offline", b"allow:/tmp/o"] {
        let p = parse(word).unwrap();
        assert_eq!((connected(&p, &[]), net_line(&p, &[])), (None, None));
        let e = End { exit: 0, not_started: None, reached: &[] };
        let text = report(&p, &box_hash(&p), &Tally::default(), None, &e);
        assert!(!text.contains("connected"), "{text}");
    }
}
