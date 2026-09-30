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
 * A kept record whose name or tier setup would not have written is refused,
 * and the refusal names the part that was wrong.
 */

use nonos_policy_proto::setup_record::{check, Answers, Name, Refused, Tier, ANSWERS_V2_LEN};

const NAME_AT: usize = 7;
const TIER_AT: usize = NAME_AT + 1 + 32;

fn raw() -> [u8; ANSWERS_V2_LEN] {
    let a = Answers {
        keyboard_layout: 0,
        timezone: 1,
        wallpaper: 0,
        username: Name::new(b"ada").unwrap(),
        qwen_tier: Tier::new(b"small").unwrap(),
    };
    a.encode()
}

#[test]
fn a_malformed_name_is_refused_by_name() {
    let mut r = raw();
    r[NAME_AT + 1] = b'A';
    assert_eq!(check(&r), Err(Refused::Name));
    let mut r = raw();
    r[NAME_AT] = 33;
    assert_eq!(check(&r), Err(Refused::Name));
    let mut r = raw();
    r[NAME_AT + 1 + 4] = b'x';
    assert_eq!(check(&r), Err(Refused::Name), "bytes past the name's end");
    assert_eq!(Refused::Name.name(), "name");
}

#[test]
fn a_malformed_tier_is_refused_by_name() {
    let mut r = raw();
    r[TIER_AT + 1] = b'/';
    assert_eq!(check(&r), Err(Refused::Tier));
    let mut r = raw();
    r[TIER_AT] = 25;
    assert_eq!(check(&r), Err(Refused::Tier));
    assert_eq!(Refused::Tier.name(), "qwen tier");
}

#[test]
fn only_names_and_tiers_setup_takes_can_be_made() {
    assert!(Name::new(b"1ada").is_none());
    assert!(Name::new(&[b'a'; 33]).is_none());
    assert!(Tier::new(b"Qwen").is_none());
    assert!(Tier::new(&[b'a'; 25]).is_none());
}
