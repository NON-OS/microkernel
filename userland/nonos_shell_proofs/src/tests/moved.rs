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

//! The builtins moved out of the terminal print what they printed there and
//! return the same status.

use crate::kit::{argv, Lines, Script};
use nonos_shell::builtin::{b64, echo, hash, hex, printf, rand, seq};

fn out_of(f: impl FnOnce(&mut Lines) -> bool) -> (bool, Vec<String>) {
    let mut out = Lines::default();
    let ok = f(&mut out);
    (ok, out.text())
}

#[test]
fn echo_joins_its_words_and_stops_at_the_line_width() {
    let mut out = Lines::default();
    echo::run(&mut out, &argv("echo nsh  42"));
    echo::run(&mut out, &argv("echo"));
    let long = vec![b'x'; 2000];
    echo::run(&mut out, &[b"echo", &long]);
    assert_eq!(out.0[0], b"nsh  42");
    assert_eq!(out.0[1], b"");
    assert_eq!(out.0[2].len(), echo::LINE_MAX);
}

#[test]
fn seq_counts_up_down_and_refuses_a_zero_step() {
    assert_eq!(
        out_of(|o| seq::run(o, &argv("seq 3"))),
        (true, vec!["1".into(), "2".into(), "3".into()])
    );
    assert_eq!(
        out_of(|o| seq::run(o, &argv("seq 5 -2 1"))),
        (true, vec!["5".into(), "3".into(), "1".into()])
    );
    assert_eq!(
        out_of(|o| seq::run(o, &argv("seq 1 0 3"))),
        (false, vec!["seq: step may not be zero".into()])
    );
    assert!(!out_of(|o| seq::run(o, &argv("seq x"))).0);
}

#[test]
fn hex_and_b64_round_trip() {
    assert_eq!(out_of(|o| hex::run(o, &argv("hex nsh"))), (true, vec!["6e7368".into()]));
    assert_eq!(out_of(|o| hex::run(o, &argv("hex -d 6e7368"))), (true, vec!["nsh".into()]));
    assert_eq!(out_of(|o| b64::run(o, &argv("b64 nsh"))), (true, vec!["bnNo".into()]));
    assert_eq!(out_of(|o| b64::run(o, &argv("b64 -d bnNo"))), (true, vec!["nsh".into()]));
}

#[test]
fn hash_is_the_digest_of_the_joined_words() {
    let want: String = nonos_hash::sha256(b"a b").iter().map(|b| format!("{b:02x}")).collect();
    assert_eq!(out_of(|o| hash::run(o, &argv("hash a b"))), (true, vec![want]));
    assert!(!out_of(|o| hash::run(o, &argv("hash -a md5 x"))).0);
}

#[test]
fn rand_draws_from_the_host_and_fails_when_it_refuses() {
    let mut host = Script { random: Some(0xAB), ..Script::default() };
    assert_eq!(out_of(|o| rand::run(o, &mut host, &argv("rand 2"))), (true, vec!["abab".into()]));
    let mut refusing = Script::default();
    assert!(!out_of(|o| rand::run(o, &mut refusing, &argv("rand"))).0);
    assert!(!out_of(|o| rand::run(o, &mut host, &argv("rand 257"))).0);
}

#[test]
fn printf_formats_and_splits_on_newlines() {
    let (ok, lines) = out_of(|o| printf::run(o, &[b"printf", b"%s=%d\n%x", b"n", b"42", b"255"]));
    assert!(ok);
    assert_eq!(lines, vec!["n=42".to_string(), "ff".to_string()]);
}
