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
 * A kept setup record reads back as written, in either version, so a record
 * an earlier build kept still restores after an update.
 */

use nonos_policy_proto::setup_record::{Answers, Name, Tier};
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

#[test]
fn the_current_version_keeps_name_and_tier() {
    let a = kept(b"ada_l-1", b"qwen3-1.7b");
    let raw = a.encode();
    assert_eq!(raw.len(), ANSWERS_LEN);
    assert_eq!(&raw[..4], b"NSA2");
    assert_eq!(Answers::decode(&raw), Some(a));
    assert_eq!(Answers::decode(&raw).unwrap().username.as_bytes(), b"ada_l-1");
    assert_eq!(Answers::decode(&raw).unwrap().qwen_tier.as_bytes(), b"qwen3-1.7b");
}

#[test]
fn empty_name_and_tier_round_trip() {
    let a = kept(b"", b"");
    assert_eq!(Answers::decode(&a.encode()), Some(a));
}

#[test]
fn the_longest_name_and_tier_fit() {
    let a = kept(&[b'a'; 32], &[b'x'; 24]);
    assert_eq!(Answers::decode(&a.encode()), Some(a));
}

#[test]
fn an_old_record_still_loads_without_name_or_tier() {
    let raw: [u8; ANSWERS_V1_LEN] = *b"NSA1\x02\xfb\x03";
    assert_eq!(Answers::decode(&raw), Some(kept(b"", b"")));
    assert_eq!(kept(b"ada", b"max").encode_v1(), raw);
}
