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

//! Every refusal or end the server reports is said on the serial line, as
//! one line naming the whole `Why`, and nothing else from the event.

use crate::refused_said::line;

const SHELL: &str = include_str!("../../../capsule_nsh_front/src/shell.rs");
const WHY: &str = include_str!("../../../nsh_server/src/why.rs");

// The shapes of nsh_server's Why, for the line's text: a kind, and a kind
// holding a kind or a number.
#[allow(dead_code)]
#[derive(Debug)]
enum Refusal {
    Expired,
}
#[allow(dead_code)]
#[derive(Debug)]
enum Fault {
    Kernel(i64),
}
#[allow(dead_code)]
#[derive(Debug)]
enum Why {
    HandshakeTime,
    Login(Refusal),
    Attest(Fault),
}

#[test]
fn the_line_names_the_whole_why() {
    assert_eq!(line(&Why::HandshakeTime), b"[NSH-FRONT] refused or ended: HandshakeTime\n");
    assert_eq!(
        line(&Why::Login(Refusal::Expired)),
        b"[NSH-FRONT] refused or ended: Login(Expired)\n"
    );
    assert_eq!(
        line(&Why::Attest(Fault::Kernel(-1))),
        b"[NSH-FRONT] refused or ended: Attest(Kernel(-1))\n"
    );
}

#[test]
fn it_is_one_line() {
    for why in
        [Why::HandshakeTime, Why::Login(Refusal::Expired), Why::Attest(Fault::Kernel(i64::MIN))]
    {
        let text = line(&why);
        assert_eq!(text.iter().filter(|&&b| b == b'\n').count(), 1);
        assert_eq!(text.last(), Some(&b'\n'));
    }
}

#[test]
fn every_refused_event_is_said() {
    let refused = SHELL.find("S::Refused(why) => {").expect("the shell takes Refused");
    let said = SHELL.find("refused_said::line(why)").expect("its line");
    let debug = SHELL.find("mk_debug(text.as_ptr(), text.len())").expect("written");
    assert!(refused < said && said < debug);
    assert!(refused < SHELL.find("_ => {}").expect("the rest"), "before the catch-all");
}

#[test]
fn why_holds_no_bytes_from_the_peer() {
    // A Vec, a slice or a String in Why could carry what the peer sent onto
    // the serial line; every variant holds a kind or a number only.
    let body =
        &WHY[WHY.find("pub enum Why {").expect("Why")..WHY.find("impl Why").expect("its impl")];
    for held in ["Vec<", "&[u8]", "String", "[u8;"] {
        assert!(!body.contains(held), "Why holds {held}");
    }
}
