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

//! Signing from the command line: a body other than the one `body` made,
//! a body that is not one, and a key file that is not one are refused, and
//! nothing is written.

mod common;
use common::{fresh, run, said};

#[test]
fn a_bad_body_or_a_certificate_under_another_key_is_refused() {
    let d = fresh("sign");
    assert!(run(&d, &["keygen", "--out", "."]).status.success());
    std::fs::write(d.join("body"), b"NSHE not a body").expect("written");
    let digest = nsh_enrol::hex::of(&nonos_hash::sha256(b"NSHE not a body"));
    let sign = |key: &str, expect: &str| {
        run(&d, &["sign", "--body", "body", "--expect", expect, "--key", key, "--out", "c"])
    };
    let o = sign("authority.secret", &"00".repeat(32));
    assert!(said(&o).contains("not the one") && !d.join("c").exists(), "{}", said(&o));
    let o = sign("authority.secret", &digest);
    assert!(said(&o).contains("certificate body") && !d.join("c").exists(), "{}", said(&o));
    let o = sign("authority.public", &digest);
    assert!(said(&o).contains("key file"), "{}", said(&o));
    let o = run(&d, &["show", "--cert", "body", "--authority", "authority.public"]);
    assert!(said(&o).contains("does not verify"), "{}", said(&o));
}
