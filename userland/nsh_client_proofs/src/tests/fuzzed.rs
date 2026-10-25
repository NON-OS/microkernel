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

//! The fuzz harness's own proof: a `HOST_PROOF` built from given fields is
//! answered with `USER_AUTH` exactly when the oracle says every check
//! passes. All zeros is the good proof; each other input changes one field.

use crate::kit::fuzz_run::host_run;

fn agrees(b: &[u8]) -> bool {
    let run = host_run(b);
    assert_eq!(run.credential, run.oracle.all(), "{b:?}: {:?}", run.oracle);
    assert_eq!(run.oracle.all(), run.pair.client.attester().said_ok.get() && run.credential);
    run.credential
}

#[test]
fn the_good_fields_log_in_and_each_change_is_judged_alike() {
    assert!(agrees(&[]), "all zeros is the good proof");
    let mut refused = 0;
    for i in 0..16 {
        for v in [1u8, 0x80] {
            let mut b = vec![0u8; 16];
            b[i] = v;
            refused += usize::from(!agrees(&b));
        }
    }
    assert!(refused >= 20, "most single changes are refused: {refused}");
    let run = host_run(&[0x90]);
    assert!(run.oracle.host_sig, "the stranger's keys, signed by the stranger, pass the signature");
    assert!(!run.credential, "but the enrolment names other host keys");
}
