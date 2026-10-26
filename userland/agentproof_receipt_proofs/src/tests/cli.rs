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
//! A whole receipt verifies, and says what it carries.
//! What `agentproof-receipt verify` prints and exits with, and the trust
//! file it reads.

use agentproof_receipt::cli::{answer, hex};
use agentproof_receipt::trust::Trust;
use agentproof_receipt::trust_file::read;

use crate::fixture::Fixture;

pub fn written(t: &Trust) -> String {
    let mut s = String::from("nonos-receipt-trust 1\n# the fixture's\n");
    for r in &t.releases {
        let line = [r.version.clone(), hex(&r.pcr8), hex(&r.pcr9)].join(" ");
        s += &format!("release {line}\n");
    }
    t.supervisors.iter().for_each(|v| s += &format!("supervisor {} {:#x}\n", hex(&v.meas), v.caps));
    s + &format!("ak {}\n", hex(&t.ak))
}

#[test]
fn a_verified_receipt_is_said_line_by_line_and_exits_0() {
    let mut f = Fixture::new(b"learn");
    let (bytes, trust) = (f.bytes(), f.trust());
    assert_eq!(read(&written(&trust)).unwrap(), trust);
    let a = answer(&bytes, &written(&trust));
    assert_eq!((a.code, a.err.len()), (0, 0));
    assert!(a.out[0].starts_with("receipt  verified: NAPR v0; the AK is the one pinned"));
    assert!(a.out.iter().any(|l| l.starts_with("box      learn ")));
    assert!(a.out.iter().any(|l| l.starts_with("mode     LEARN, not enforced")));
    assert!(a.out.iter().any(|l| l.starts_with("ran      3 capsules, every one the signed")));
    assert!(a.out.iter().any(|l| l.starts_with("network  pid 5 ")), "{:?}", a.out);
    assert_eq!(a.out.last().unwrap(), "counts   not carried in v0");
}

#[test]
fn a_refusal_exits_1_by_name_and_a_bad_trust_file_2() {
    let mut f = Fixture::new(b"strict");
    let (mut bytes, trust) = (f.bytes(), f.trust());
    bytes[30] ^= 1;
    let a = answer(&bytes, &written(&trust));
    assert_eq!(a.code, 1);
    assert!(a.err[0].starts_with("agentproof-receipt: refused: challenge"), "{:?}", a.err);
    for (text, why) in [
        ("", "the first line is `nonos-receipt-trust 1`"),
        ("nonos-receipt-trust 1\nak 00\n", "the AK is 128 hex digits"),
        ("nonos-receipt-trust 1\nlisten 1\n", "a line is release, supervisor, ak or a # comment"),
        ("nonos-receipt-trust 1\n", "no ak line"),
    ] {
        let a = answer(&bytes, text);
        assert_eq!(a.code, 2, "{text:?}");
        assert!(a.err[0].ends_with(why), "{:?}", a.err);
    }
}
