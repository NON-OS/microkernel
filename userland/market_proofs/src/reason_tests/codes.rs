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
const PAIRS: [(Why, u8); 33] = [
    (Why::Index, WHY_INDEX),
    (Why::NotProvided, WHY_NOT_PROVIDED),
    (Why::TooLarge, WHY_TOO_LARGE),
    (Why::Package, WHY_PACKAGE),
    (Why::NoMirror, WHY_NO_MIRROR),
    (Why::NoKeyring, WHY_NO_KEYRING),
    (Why::NoVolume, WHY_NO_VOLUME),
    (Why::VolumeLocked, WHY_VOLUME_LOCKED),
    (Why::NoNetwork, WHY_NO_NETWORK),
    (Why::ModelUnknown, WHY_MODEL_UNKNOWN),
    (Why::ModelMismatch, WHY_MODEL_MISMATCH),
    (Why::ModelFetch, WHY_MODEL_FETCH),
    (Why::NoRoom, WHY_NO_ROOM),
    (Why::ModelBusy, WHY_MODEL_BUSY),
    (Why::ModelUnkept, WHY_MODEL_UNKEPT),
    (Why::ModelTooLarge, WHY_MODEL_TOO_LARGE),
    (Why::VolumeFailed, WHY_VOLUME_FAILED),
    (Why::Partial, WHY_PARTIAL),
    (Why::NotInstalled, WHY_NOT_INSTALLED),
    (Why::ModelKept, WHY_MODEL_KEPT),
    (Why::RemovePartial, WHY_REMOVE_PARTIAL),
    (Why::NoMemory, WHY_NO_MEMORY),
    (Why::ModelDownloading, WHY_MODEL_DOWNLOADING),
    (Why::BootOffline, WHY_BOOT_OFFLINE),
    (Why::NymUnreachable, WHY_NYM_UNREACHABLE),
    (Why::AnyoneUnreachable, WHY_ANYONE_UNREACHABLE),
    (Why::MirrorUnreachable, WHY_MIRROR_UNREACHABLE),
    (Why::NymNoExit, WHY_NYM_NO_EXIT),
    (Why::AnyoneNoExit, WHY_ANYONE_NO_EXIT),
    (Why::AnyoneNotUp, WHY_ANYONE_NOT_UP),
    (Why::ProgramMissing, WHY_PROGRAM_MISSING),
    (Why::ProgramUnreadable, WHY_PROGRAM_UNREADABLE),
    (Why::UnpackMemory, WHY_UNPACK_MEMORY),
];

#[test]
fn the_clients_read_the_codes_the_personality_exits_with() {
    for (i, (why, code)) in PAIRS.iter().enumerate() {
        assert_eq!(why.code(), i32::from(*code), "{why:?}");
        assert_ne!(reason(*code).line, reason(u8::MAX).line, "code {code} has no sentence");
        for (_, other) in &PAIRS[i + 1..] {
            assert_ne!(reason(*code).line, reason(*other).line, "{code} and {other}");
        }
    }
}
