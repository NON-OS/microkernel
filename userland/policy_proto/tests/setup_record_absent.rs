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

/*
 * What is not a record setup wrote: withdrawn zeros, a magic that does not
 * match its version's length, and a time zone setup does not offer.
 */

use nonos_policy_proto::setup_record::{check, Answers, Name, Refused, Tier};
use nonos_policy_proto::setup_record::{ANSWERS_LEN, ANSWERS_V1_LEN};

fn kept(name: &[u8], tier: &[u8]) -> Answers {
    Answers {
        keyboard_layout: 2,
        timezone: -5,
        wallpaper: 3,
        username: Name::new(name).unwrap(),
        qwen_tier: Tier::new(tier).unwrap(),
    }
}

fn raw() -> [u8; ANSWERS_LEN] {
    kept(b"ada", b"small").encode()
}

#[test]
fn zeros_of_either_length_read_as_absent() {
    assert_eq!(check(&[0; ANSWERS_V1_LEN]), Err(Refused::Magic));
    assert_eq!(check(&[0; ANSWERS_LEN]), Err(Refused::Magic));
    assert_eq!(check(b""), Err(Refused::Length));
}

#[test]
fn magic_must_match_its_version_length() {
    let mut raw = kept(b"ada", b"max").encode().to_vec();
    raw[3] = b'1';
    assert_eq!(check(&raw), Err(Refused::Magic));
    assert_eq!(check(&b"NSA2\x02\xfb\x03"[..]), Err(Refused::Magic));
    raw.push(0);
    assert_eq!(check(&raw), Err(Refused::Length));
}

#[test]
fn a_time_zone_setup_does_not_offer_is_refused() {
    let mut r = raw();
    r[5] = 0x7f;
    assert_eq!(check(&r), Err(Refused::Timezone));
    assert_eq!(check(b"NSA1\0\x7f\0"), Err(Refused::Timezone));
}
