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

//! The builtins added for nsh: basename, dirname, diff, date, uptime, mem
//! and ed25519, through a recording sink and a scripted host.

use crate::kit::{argv, Lines, Script};
use nonos_shell::builtin::{clock, diff, ed25519, mem, path};
use nonos_shell::host::Mem;

#[test]
fn basename_and_dirname_read_a_path_as_posix_does() {
    let cases: &[(&[u8], &[u8], &[u8])] = &[
        (b"/usr/lib/x.so", b"x.so", b"/usr/lib"),
        (b"/usr/lib/", b"lib", b"/usr"),
        (b"x", b"x", b"."),
        (b"/", b"/", b"/"),
        (b"//a", b"a", b"/"),
        (b"a/b//", b"b", b"a"),
        (b"", b"", b"."),
    ];
    for (p, b, d) in cases {
        assert_eq!((path::base(p, b""), path::dir(p)), (*b, *d), "{}", String::from_utf8_lossy(p));
    }
    assert_eq!(path::base(b"/a/x.tar", b".tar"), b"x");
    assert_eq!(path::base(b".tar", b".tar"), b".tar", "a suffix that is the whole name stays");
    let mut out = Lines::default();
    assert!(path::basename(&mut out, &argv("basename /a/b.rs .rs")));
    assert!(!path::dirname(&mut out, &argv("dirname")));
    assert_eq!(out.text(), ["b", "usage: dirname path"]);
}

#[test]
fn date_writes_the_wall_clock_as_utc() {
    assert_eq!(clock::utc(0), b"1970-01-01 00:00:00 UTC");
    assert_eq!(clock::utc(951_782_400), b"2000-02-29 00:00:00 UTC");
    assert_eq!(clock::utc(1_793_863_742), b"2026-11-05 07:29:02 UTC");
    let mut host = Script { wall_ms: Some(1_793_863_742_450), ..Script::default() };
    let mut out = Lines::default();
    assert!(clock::date(&mut out, &mut host));
    assert!(!clock::date(&mut out, &mut Script::default()));
    assert_eq!(out.text(), ["2026-11-05 07:29:02 UTC", "date: clock unavailable"]);
}

#[test]
fn uptime_and_mem_say_what_the_host_knows() {
    let mut host = Script {
        uptime_ms: Some(3_723_999),
        mem: Some(Mem { total: 4 << 30, used: 1 << 30 }),
        ..Script::default()
    };
    let mut out = Lines::default();
    assert!(clock::uptime(&mut out, &mut host));
    assert!(mem::run(&mut out, &mut host));
    assert!(!mem::run(&mut out, &mut Script::default()));
    assert_eq!(
        out.text(),
        ["up 1h 2m 3s", "mem: 1048576 KiB used of 4194304 KiB (25%)", "mem: unavailable"]
    );
    let mut zero = Script { mem: Some(Mem { total: 0, used: 0 }), ..Script::default() };
    let mut out = Lines::default();
    assert!(mem::run(&mut out, &mut zero));
    assert_eq!(out.text(), ["mem: 0 KiB used of 0 KiB"], "no share of nothing");
}

fn files(pairs: &[(&str, &str)]) -> Script {
    let mut s = Script::default();
    for (n, b) in pairs {
        s.files.insert(n.as_bytes().to_vec(), b.as_bytes().to_vec());
    }
    s
}

#[test]
fn diff_names_each_line_only_one_file_has() {
    let mut host = files(&[
        ("a", "one\ntwo\nthree\n"),
        ("b", "one\n2\nthree\nfour\n"),
        ("c", "one\ntwo\nthree\n"),
    ]);
    let mut out = Lines::default();
    assert!(!diff::run(&mut out, &mut host, &argv("diff a b")));
    assert_eq!(out.text(), ["< two", "> 2", "> four"]);
    let mut out = Lines::default();
    assert!(diff::run(&mut out, &mut host, &argv("diff a c")), "equal files succeed");
    assert!(out.0.is_empty());
}

#[test]
fn diff_refuses_what_it_cannot_hold() {
    let big = "x\n".repeat(diff::MAX_LINES + 1);
    let mut host = files(&[("a", "x\n"), ("big", &big)]);
    let mut out = Lines::default();
    assert!(!diff::run(&mut out, &mut host, &argv("diff a big")));
    assert!(!diff::run(&mut out, &mut host, &argv("diff a gone")));
    assert!(!diff::run(&mut out, &mut host, &argv("diff a")));
    assert_eq!(
        out.text(),
        [
            "diff: big: over 512 lines",
            "diff: gone: cannot read it, or it is over 64 KiB",
            "usage: diff file1 file2"
        ]
    );
}

#[test]
fn ed25519_keygen_sign_and_verify_agree() {
    let mut host = Script { random: Some(7), ..Script::default() };
    let mut out = Lines::default();
    assert!(ed25519::run(&mut out, &mut host, &argv("ed25519 keygen")));
    let seed = out.text()[0].strip_prefix("seed ").expect("a seed").to_string();
    let public = out.text()[1].strip_prefix("public ").expect("a public key").to_string();
    assert_eq!(seed, "07".repeat(32));
    let mut out = Lines::default();
    assert!(ed25519::run(&mut out, &mut host, &argv(&format!("ed25519 sign {seed} hello nsh"))));
    let sig = out.text()[0].clone();
    assert_eq!(sig.len(), 128);
    let mut out = Lines::default();
    assert!(ed25519::run(
        &mut out,
        &mut host,
        &argv(&format!("ed25519 verify {public} {sig} hello nsh"))
    ));
    assert!(!ed25519::run(
        &mut out,
        &mut host,
        &argv(&format!("ed25519 verify {public} {sig} hello NSH"))
    ));
    assert_eq!(out.text(), ["ed25519: good signature", "ed25519: bad signature"]);
}

#[test]
fn ed25519_matches_rfc_8032_test_1() {
    let seed = "9d61b19deffd5a60ba844af492ec2cc44449c5697b326919703bac031cae7f60";
    let want = "e5564300c360ac729086e2cc806e828a84877f1eb8e5d974d873e065224901555fb8821590a33bacc61e39701cf9b46bd25bf5f0595bbe24655141438e7a100b";
    let mut out = Lines::default();
    assert!(ed25519::run(
        &mut out,
        &mut Script::default(),
        &[b"ed25519", b"sign", seed.as_bytes()]
    ));
    assert_eq!(out.text(), [want]);
}

#[test]
fn ed25519_refuses_bad_keys_and_a_refusing_generator() {
    let mut out = Lines::default();
    let mut host = Script::default();
    assert!(!ed25519::run(&mut out, &mut host, &argv("ed25519 keygen")));
    assert!(!ed25519::run(&mut out, &mut host, &argv("ed25519 sign 00 x")));
    assert!(!ed25519::run(
        &mut out,
        &mut host,
        &argv(&format!("ed25519 verify {} zz x", "00".repeat(32)))
    ));
    assert!(!ed25519::run(&mut out, &mut host, &argv("ed25519 frobnicate")));
}
