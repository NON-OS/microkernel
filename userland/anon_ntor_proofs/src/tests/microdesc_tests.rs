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

//! Microdescriptors, including the RSA key the fork made optional.

use crate::directory::microdesc;
use crate::vectors::MICRODESCS;

#[test]
fn live_microdescriptors_parse_and_carry_both_keys() {
    let pieces = microdesc::pieces(MICRODESCS);
    assert_eq!(pieces.len(), 3, "three were fetched");
    for (start, end) in pieces {
        let parsed = microdesc::parse(&MICRODESCS[start..end]).expect("parses");
        assert!(parsed.ntor_onion_key.iter().any(|b| *b != 0));
        assert!(parsed.ed25519_identity.iter().any(|b| *b != 0));
    }
}

#[test]
fn an_rsa_onion_key_is_present_and_ignored() {
    assert!(MICRODESCS.starts_with(b"onion-key\n-----BEGIN RSA PUBLIC KEY-----"));
    let without = b"ntor-onion-key 4k42YwphMtr96Q1qlyhy6O9RB6h9xzkmtxEMqd3mU3I\n\
                    id ed25519 lzrs8z5HT9mg13o5JX1V3cED9ZSk8HbMUMtxrbrY3OM\n\
                    p reject 23,25\n";
    let parsed = microdesc::parse(without).expect("one with no RSA key still parses");
    assert!(parsed.exits_web, "reject 23,25 leaves 80 and 443 open");
}

#[test]
fn a_microdescriptor_without_an_ed25519_identity_is_refused() {
    let no_id = b"ntor-onion-key 4k42YwphMtr96Q1qlyhy6O9RB6h9xzkmtxEMqd3mU3I\np reject 23\n";
    assert!(microdesc::parse(no_id).is_none(), "EXTEND2 needs the Ed25519 identity");
}

#[test]
fn the_exit_summary_respects_its_verb() {
    assert!(microdesc::policy::exits_web(b"reject 23,25,465,587"));
    assert!(!microdesc::policy::exits_web(b"reject 23,80,443"));
    assert!(!microdesc::policy::exits_web(b"reject 1-65535"));
    assert!(microdesc::policy::exits_web(b"accept 80,443"));
    assert!(!microdesc::policy::exits_web(b"accept 80"), "443 is required as well");
    assert!(microdesc::policy::exits_web(b"accept 79-444"));
    assert!(!microdesc::policy::exits_web(b"nonsense 80,443"));
}
