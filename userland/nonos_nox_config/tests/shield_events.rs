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

mod keccak;

use keccak::keccak;
use nonos_nox_config::{NOTE_COMMITTED, SHIELD_EVENTS};

#[test]
fn every_shield_topic_is_its_declarations_hash() {
    for (signature, topic) in SHIELD_EVENTS {
        assert_eq!(keccak(signature.as_bytes()), topic, "{signature}");
    }
    let mut all: Vec<[u8; 32]> = SHIELD_EVENTS.iter().map(|(_, t)| *t).collect();
    all.sort();
    all.dedup();
    assert_eq!(all.len(), SHIELD_EVENTS.len());
    assert_eq!(NOTE_COMMITTED[..4], [0x4d, 0x30, 0x7f, 0xe1], "shield_core's NOTE_COMMITTED");
}
