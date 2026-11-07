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
//! `whoami` prints the login a session was admitted as, and refuses when
//! nothing admitted one.

use crate::kit::{Lines, Script};
use nonos_shell::session::{Session, Who, NO_LOGIN};

fn run(s: &mut Session, line: &str) -> Vec<String> {
    let mut out = Lines::default();
    s.run_line(line.as_bytes(), &mut out, &mut Script::default());
    out.text()
}

#[test]
fn whoami_prints_the_admitted_login() {
    let mut subject = [0u8; 64];
    subject[0] = 0xab;
    subject[63] = 0x01;
    let mut s = Session::default();
    s.who = Some(Who { user: b"alice".to_vec(), role: b"ops".to_vec(), subject, caps: vec![] });
    let key = format!("key sha512 ab{}01", "00".repeat(62));
    assert_eq!(run(&mut s, "whoami; echo $?"), ["user alice", "role ops", key.as_str(), "0"]);
}

#[test]
fn whoami_with_no_login_fails() {
    let mut s = Session::default();
    assert_eq!(
        run(&mut s, "whoami; echo $?"),
        [String::from_utf8_lossy(NO_LOGIN).into_owned(), "1".into()]
    );
}

#[test]
fn whoami_never_prints_a_control_byte() {
    let mut s = Session::default();
    s.who = Some(Who {
        user: b"a\x1b[2Jb".to_vec(),
        role: b"r\x07\xff".to_vec(),
        subject: [0; 64],
        caps: vec![],
    });
    let out = run(&mut s, "whoami");
    assert_eq!(&out[..2], ["user a?[2Jb", "role r??"]);
}
