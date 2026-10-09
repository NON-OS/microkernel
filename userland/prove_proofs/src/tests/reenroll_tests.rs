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

//! A key enrolled again replaces its commitment. A transcript that lists the
//! key twice gives the root of the last line, and that line is the one read.

use nonos_device_attest::{commit, ek_id, words_of};

use crate::assemble::error::Refusal;
use crate::fixture::{area, device, secret_of};

#[test]
fn the_last_line_for_a_key_is_the_enrolled_one() {
    let mut d = device();
    let hex: String = ek_id(area(&d.eks[1])).iter().map(|b| format!("{b:02x}")).collect();
    let old = commit(&words_of(&secret_of([5, 6, 7, 8])));
    let line = format!(
        "device {hex} {} {} {} {}\n",
        old[0].to_u64(),
        old[1].to_u64(),
        old[2].to_u64(),
        old[3].to_u64()
    );
    let text = String::from_utf8(d.transcript.clone()).expect("text");
    d.transcript = text.replacen("depth 4\n", &format!("depth 4\n{line}"), 1).into_bytes();
    assert!(d.assemble().is_ok(), "{:?}", d.assemble().err());
    d.secret = secret_of([5, 6, 7, 8]);
    assert_eq!(d.assemble().err(), Some(Refusal::CommitmentMismatch));
    let upper = text.replacen(&hex, &hex.to_uppercase(), 1);
    let mut e = device();
    e.transcript = upper.into_bytes();
    assert!(e.assemble().is_ok(), "a key in upper-case hex is the same key");
}
