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

//! Why an install stopped, from the installer's exit code to the button: a
//! live boot offers no Retry, a volume that failed does.

use nonos_market_proto::reason;
use nonos_market_proto::reason::*;

use crate::linux_why::why::Why;
use crate::store::progress::Progress;

/// Every reason the personality exits with, beside the code the clients read.
const PAIRS: [(Why, u8); 32] = [
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

#[test]
fn no_data_volume_is_a_live_boot_and_offers_no_retry() {
    let said = reason(WHY_NO_VOLUME);
    assert!(!said.retry);
    // Only a machine with no NONOS disk at all: a live stick's volume is in memory.
    assert!(said.line.contains("no NONOS disk at all") && said.line.contains("Install NONOS"));
    let p = Progress::Failed(WHY_NO_VOLUME);
    assert!(p.needs_installed_system() && !p.retryable());
    assert_eq!(p.button(true), b"Live boot");
}

#[test]
fn a_volume_that_failed_offers_a_retry_and_says_so_honestly() {
    let said = reason(WHY_VOLUME_FAILED);
    assert!(said.retry);
    // The kernel answers EIO for a machine with no NONOS disk too, today.
    assert!(said.line.contains("retry") && said.line.contains("no NONOS disk"), "{}", said.line);
    let p = Progress::Failed(WHY_VOLUME_FAILED);
    assert!(p.retryable() && !p.needs_installed_system());
    assert_eq!(p.button(true), b"Retry");
}

#[test]
fn what_the_system_lacks_offers_no_retry() {
    for code in [
        WHY_BOOT_OFFLINE,
        WHY_TOO_LARGE,
        WHY_NO_MIRROR,
        WHY_NO_KEYRING,
        WHY_MODEL_UNKNOWN,
        WHY_MODEL_UNKEPT,
        WHY_MODEL_TOO_LARGE,
    ] {
        assert!(!reason(code).retry, "code {code}");
        assert_eq!(Progress::Failed(code).button(true), b"Details", "code {code}");
    }
}

#[test]
fn a_package_that_did_not_land_whole_is_a_failure_with_a_retry() {
    let p = Progress::Failed(WHY_PARTIAL);
    assert!(p != Progress::Installed && p.retryable());
    assert!(reason(WHY_PARTIAL).line.contains("none of it was kept"));
}

#[test]
fn an_unknown_stop_and_a_refusal_may_be_retried() {
    assert!(reason(0).retry && reason(200).retry);
    assert!(Progress::Refused.retryable());
    assert_eq!(Progress::Refused.button(false), b"Retry");
}

/*
 * A live session's volume is held in memory. Memory filling (ENOSPC) is
 * mended by freeing memory and retrying; too little to hold a volume at all
 * (ENOMEM) is not mended by asking again.
 */
#[test]
fn a_live_session_out_of_memory_is_said_and_only_enospc_retries() {
    let room = reason(WHY_NO_ROOM);
    assert!(room.retry && room.line.contains("free memory") && room.line.contains("install NONOS"));
    let memory = reason(WHY_NO_MEMORY);
    assert!(!memory.retry && memory.line.contains("free memory"), "{}", memory.line);
    assert_eq!(Progress::Failed(WHY_NO_MEMORY).button(true), b"Details");
    assert_eq!(Progress::Failed(WHY_NO_ROOM).button(true), b"Retry");
}

/* An install is never said to be kept where it is held in memory. */
#[test]
fn installed_says_where_it_is_held() {
    use nonos_market_proto::installed_line;
    let tier = installed_line(b"linux.qwen-small");
    assert!(tier.contains("in memory") && tier.contains("power off"), "{tier}");
    let package = installed_line(b"linux.jq");
    assert!(package.contains("for this session") && package.contains("restart"), "{package}");
    for line in [tier, package] {
        assert!(!line.to_lowercase().contains("kept"), "{line}");
    }
}

/* An uninstall is followed through: removing, then as good as never installed. */
#[test]
fn an_uninstall_reads_as_removing_then_not_installed() {
    use nonos_market_proto::{removal_only, Stage};
    assert_eq!(Stage::of(5), Stage::Removing);
    assert_eq!(Stage::of(6), Stage::Removed);
    assert!(Stage::Removing.pending() && !Stage::Removed.pending());
    assert_eq!(Progress::of(5), Progress::Removing);
    assert!(Progress::Removing.pending());
    assert_eq!(Progress::Removing.button(true), b"Removing");
    assert_eq!(Progress::of(6), Progress::Removed);
    assert_eq!(Progress::Removed.button(true), b"Install");
    assert!(!Progress::Removed.retryable());
    for code in [WHY_NOT_INSTALLED, WHY_MODEL_KEPT, WHY_REMOVE_PARTIAL, WHY_MODEL_DOWNLOADING] {
        assert!(removal_only(code), "{code}");
    }
    assert!(!removal_only(WHY_VOLUME_FAILED) && !removal_only(WHY_PACKAGE));
    let kept = reason(WHY_MODEL_KEPT);
    assert!(!kept.retry && kept.line.contains("without the model fetcher"), "{}", kept.line);
}
