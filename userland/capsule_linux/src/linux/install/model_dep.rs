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
 * A shipped tier's model as a dependency of its install. The tier's table
 * names the files (`App::models`), each pinned by SHA-256 in the signed
 * personality and all of one tier of the signed model catalogue
 * (`App::tier`). A model is never looked up in a package index: a file the
 * data volume holds under its pin is kept as it is, a missing one is handed
 * to the model fetcher, and after the fetcher every file is held to its pin
 * again before the tier counts as installed. Each refusal is the installer's
 * exit code (`Why`), which the store says in words. Pure: what the volume
 * and the fetcher answered goes in, so model_fetch_proofs holds the rules.
 */

use super::fetch_exit::{
    of_errno, ANYONE_NOT_UP, ANYONE_NO_EXIT, ANYONE_UNREACHABLE, BUSY, DONE, LOCKED, MIRROR_UNREACHABLE, MISMATCH,
    NO_CATALOGUE, NO_MEMORY, NO_NETWORK, NO_ROOM, NO_VOLUME, NYM_NO_EXIT, NYM_UNREACHABLE, REFUSED,
    TOO_LITTLE_MEMORY, UNKEPT, USAGE, VOLUME_FAILED,
};
use super::why::Why;

/* The volume's answers come to the installer positive. */
const ENOENT: i64 = 2;

/* MkToolRun's come negative. */
const NOT_BUILT: i64 = -2;
const RUNNING: i64 = -16;
const NETDOWN: i64 = -100;

/*
 * Whether the fetcher is needed, from each file's answer as `ensure` gives
 * it: the file's length once its import record is the pin, else an errno.
 * ENOENT is a file the volume lacks; any other errno is the volume's own
 * refusal, which no download would mend.
 */
pub fn needs_fetch(have: &[Result<u64, i64>]) -> Result<bool, Why> {
    let mut missing = false;
    for got in have {
        match *got {
            Ok(_) => {}
            Err(ENOENT) => missing = true,
            Err(e) => return Err(volume(e)),
        }
    }
    Ok(missing)
}

/* Why the fetcher did not start, from MkToolRun's errno. */
pub fn not_started(rc: i64) -> Why {
    match rc {
        /*
         * The boot runs no network: Air-Gapped, Safe Mode or Recovery. Not
         * the chosen network being down, which a retry may get past: nothing
         * on this boot will start the fetcher.
         */
        NETDOWN => Why::BootOffline,
        /* Built without the fetcher, so with no catalogue to list a tier. */
        NOT_BUILT => Why::ModelUnknown,
        /* One fetcher runs at a time: a `qwen get`, or another tier's install. */
        RUNNING => Why::ModelBusy,
        /* Its signed artifacts did not verify, or there was no room to start it. */
        _ => Why::ModelFetch,
    }
}

/*
 * The tier once the fetcher has ended: its exit status, then each file's
 * answer asked again. A fetcher that says it is done is not taken at its
 * word: every file must answer with its pinned length from an import record
 * that is its pin, or the tier is not installed.
 */
pub fn settle(status: i64, again: &[Result<u64, i64>]) -> Result<(), Why> {
    fetched(status)?;
    if needs_fetch(again)? {
        return Err(Why::ModelFetch);
    }
    Ok(())
}

/* The fetcher's exit status as the installer's reason. */
pub fn fetched(status: i64) -> Result<(), Why> {
    let Ok(status) = i32::try_from(status) else { return Err(Why::ModelFetch) };
    match status {
        DONE => Ok(()),
        USAGE | NO_CATALOGUE => Err(Why::ModelUnknown),
        NO_NETWORK => Err(Why::NoNetwork),
        NO_VOLUME => Err(Why::NoVolume),
        VOLUME_FAILED => Err(Why::VolumeFailed),
        NO_MEMORY => Err(Why::NoMemory),
        LOCKED => Err(Why::VolumeLocked),
        MISMATCH => Err(Why::ModelMismatch),
        NO_ROOM => Err(Why::NoRoom),
        BUSY => Err(Why::ModelBusy),
        UNKEPT => Err(Why::ModelUnkept),
        TOO_LITTLE_MEMORY => Err(Why::ModelTooLarge),
        /* The network runs and reached no mirror: said as unreachable, not as a cut download. */
        NYM_UNREACHABLE => Err(Why::NymUnreachable),
        ANYONE_UNREACHABLE => Err(Why::AnyoneUnreachable),
        MIRROR_UNREACHABLE => Err(Why::MirrorUnreachable),
        /* Exits that did not answer: not a cut download, and direct is offered. */
        NYM_NO_EXIT => Err(Why::NymNoExit),
        ANYONE_NO_EXIT => Err(Why::AnyoneNoExit),
        ANYONE_NOT_UP => Err(Why::AnyoneNotUp),
        /* No mirror served it, or it was stopped before it ended. */
        REFUSED => Err(Why::ModelFetch),
        _ => Err(Why::ModelFetch),
    }
}

/*
 * A refusal of the data volume, read as the fetcher reads the same errno.
 * Only ENODEV is a machine with no volume at all; any other refusal it does
 * not name is a volume that is there and failed, which a retry may mend.
 */
fn volume(e: i64) -> Why {
    match of_errno(e.saturating_neg()) {
        NO_VOLUME => Why::NoVolume,
        NO_MEMORY => Why::NoMemory,
        LOCKED => Why::VolumeLocked,
        NO_ROOM => Why::NoRoom,
        MISMATCH => Why::ModelMismatch,
        BUSY => Why::ModelBusy,
        _ => Why::VolumeFailed,
    }
}
