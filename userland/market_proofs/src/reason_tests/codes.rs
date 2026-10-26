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

//! Every exit reason beside the code the clients read, each with its own sentence.

use nonos_market_proto::reason;
use nonos_market_proto::reason::*;

use crate::linux_why::why::Why;

/// Every reason the personality exits with, beside the code the clients read.
const PAIRS: [(Why, u8); 35] = [
    (Why::INDEX, WHY_INDEX),
    (Why::NOT_PROVIDED, WHY_NOT_PROVIDED),
    (Why::TOO_LARGE, WHY_TOO_LARGE),
    (Why::PACKAGE, WHY_PACKAGE),
    (Why::NO_MIRROR, WHY_NO_MIRROR),
    (Why::NO_KEYRING, WHY_NO_KEYRING),
    (Why::NO_VOLUME, WHY_NO_VOLUME),
    (Why::VOLUME_LOCKED, WHY_VOLUME_LOCKED),
    (Why::NO_NETWORK, WHY_NO_NETWORK),
    (Why::MODEL_UNKNOWN, WHY_MODEL_UNKNOWN),
    (Why::MODEL_MISMATCH, WHY_MODEL_MISMATCH),
    (Why::MODEL_FETCH, WHY_MODEL_FETCH),
    (Why::NO_ROOM, WHY_NO_ROOM),
    (Why::MODEL_BUSY, WHY_MODEL_BUSY),
    (Why::MODEL_UNKEPT, WHY_MODEL_UNKEPT),
    (Why::MODEL_TOO_LARGE, WHY_MODEL_TOO_LARGE),
    (Why::VOLUME_FAILED, WHY_VOLUME_FAILED),
    (Why::PARTIAL, WHY_PARTIAL),
    (Why::NOT_INSTALLED, WHY_NOT_INSTALLED),
    (Why::MODEL_KEPT, WHY_MODEL_KEPT),
    (Why::REMOVE_PARTIAL, WHY_REMOVE_PARTIAL),
    (Why::NO_MEMORY, WHY_NO_MEMORY),
    (Why::MODEL_DOWNLOADING, WHY_MODEL_DOWNLOADING),
    (Why::BOOT_OFFLINE, WHY_BOOT_OFFLINE),
    (Why::NYM_UNREACHABLE, WHY_NYM_UNREACHABLE),
    (Why::ANYONE_UNREACHABLE, WHY_ANYONE_UNREACHABLE),
    (Why::MIRROR_UNREACHABLE, WHY_MIRROR_UNREACHABLE),
    (Why::NYM_NO_EXIT, WHY_NYM_NO_EXIT),
    (Why::ANYONE_NO_EXIT, WHY_ANYONE_NO_EXIT),
    (Why::ANYONE_NOT_UP, WHY_ANYONE_NOT_UP),
    (Why::PROGRAM_MISSING, WHY_PROGRAM_MISSING),
    (Why::PROGRAM_UNREADABLE, WHY_PROGRAM_UNREADABLE),
    (Why::POLICY, WHY_POLICY),
    (Why::UNPACK_MEMORY, WHY_UNPACK_MEMORY),
    (Why::IN_USE, WHY_IN_USE),
];

#[test]
fn the_clients_read_the_codes_the_personality_exits_with() {
    for (i, (why, code)) in PAIRS.iter().enumerate() {
        assert_eq!(why.code(), i32::from(*code), "{why:?}");
        assert_ne!(reason(*code).line, reason(u8::MAX).line, "code {code} has no sentence");
        for (other_why, other) in &PAIRS[i + 1..] {
            // Constants, not an enum's variants: nothing else keeps two apart.
            assert_ne!(why.code(), other_why.code(), "{why:?} and {other_why:?}");
            assert_ne!(reason(*code).line, reason(*other).line, "{code} and {other}");
        }
    }
}
