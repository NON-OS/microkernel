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

//! Why an install stopped, from the installer's exit code, and whether
//! asking again could come to anything.
//!
//! The codes are the Linux personality's `install::Why`; market_proofs
//! holds the two tables equal. A reason that no retry can change (a live
//! boot with no data volume, an image built without a mirror) offers no
//! Retry: a button that can only fail again is a lie.

/// The index or database did not fetch, or its signature did not verify.
pub const WHY_INDEX: u8 = 2;
/// A package, or something it depends on, is in no index.
pub const WHY_NOT_PROVIDED: u8 = 3;
/// The closure passes the configured package limit.
pub const WHY_TOO_LARGE: u8 = 4;
/// A package did not download, or did not match its pin, checksum or
/// signature.
pub const WHY_PACKAGE: u8 = 5;
/// The image was built without a mirror for this family.
pub const WHY_NO_MIRROR: u8 = 8;
/// The image was built without a keyring for this family.
pub const WHY_NO_KEYRING: u8 = 9;
/// No data volume (ENODEV): no disk carries NONOS at all.
pub const WHY_NO_VOLUME: u8 = 10;
/// The data volume is locked until its passphrase is given.
pub const WHY_VOLUME_LOCKED: u8 = 11;
/// No network to fetch a model over.
pub const WHY_NO_NETWORK: u8 = 12;
/// No signed model catalogue on this system lists the tier.
pub const WHY_MODEL_UNKNOWN: u8 = 13;
/// A model's bytes were not its signed pin; none of them was kept.
pub const WHY_MODEL_MISMATCH: u8 = 14;
/// A model did not finish downloading; what came is kept for next time.
pub const WHY_MODEL_FETCH: u8 = 15;
/// The data volume has no room for the model (ENOSPC).
pub const WHY_NO_ROOM: u8 = 16;
/// Another model download is running.
pub const WHY_MODEL_BUSY: u8 = 17;
/// A model file's name is too long for the data volume to keep.
pub const WHY_MODEL_UNKEPT: u8 = 18;
/// The tier needs more memory to run than this machine has in all.
pub const WHY_MODEL_TOO_LARGE: u8 = 19;
/// The data volume is there and could not be reached: EIO, EAGAIN.
pub const WHY_VOLUME_FAILED: u8 = 20;
/// Not every file of a package could be written into the store; what was
/// written of it was taken out again.
pub const WHY_PARTIAL: u8 = 21;
/// An uninstall found no install of the package recorded on this system.
pub const WHY_NOT_INSTALLED: u8 = 22;
/// An uninstall of a shipped tier on a system built without the model
/// fetcher, which is what takes a model off the data volume.
pub const WHY_MODEL_KEPT: u8 = 23;
/// An uninstall that left files or links behind, still recorded.
pub const WHY_REMOVE_PARTIAL: u8 = 24;
/// Too little free memory to hold a live session's data volume (ENOMEM).
pub const WHY_NO_MEMORY: u8 = 25;
/// An uninstall of a tier part of whose model is still downloading.
pub const WHY_MODEL_DOWNLOADING: u8 = 26;
/// This boot runs no network at all, so the model fetcher, which brings a
/// model in and takes one away, does not start on it.
pub const WHY_BOOT_OFFLINE: u8 = 27;
/// The chosen network runs, and no connection through it reached a model
/// mirror before a byte came: it is not reachable from this machine. Not a
/// download that started and stopped (`WHY_MODEL_FETCH`). One code for
/// each network, so the sentence names it.
pub const WHY_NYM_UNREACHABLE: u8 = 28;
pub const WHY_ANYONE_UNREACHABLE: u8 = 29;
pub const WHY_MIRROR_UNREACHABLE: u8 = 30;
/// Through Nym, or Anyone, every try in a row found no exit that answered:
/// another try may find one, and a direct download is the other choice.
pub const WHY_NYM_NO_EXIT: u8 = 31;
pub const WHY_ANYONE_NO_EXIT: u8 = 32;
/// The Anyone network, which downloads installs, did not come up in the
/// three minutes a download waits for it.
pub const WHY_ANYONE_NOT_UP: u8 = 33;
/// A shipped tier's program is not in this system's store.
pub const WHY_PROGRAM_MISSING: u8 = 34;
/// The store holds a shipped tier's program and could not read it.
pub const WHY_PROGRAM_UNREADABLE: u8 = 35;

/// Whether `code` is one only an uninstall stops with, so a client says to
/// uninstall again rather than install.
pub fn removal_only(code: u8) -> bool {
    matches!(code, WHY_NOT_INSTALLED | WHY_MODEL_KEPT | WHY_REMOVE_PARTIAL | WHY_MODEL_DOWNLOADING)
}

/// What an installed listing is, said honestly. A Linux package is held in
/// memory until restart on every boot (the store is not kept at rest). A
/// Qwen tier's model is kept on an installed system's data volume, and on
/// a live session that volume is in memory, gone at power off.
pub fn installed_line(listing: &[u8]) -> &'static str {
    match listing.starts_with(b"linux.qwen-") {
        true => {
            "Installed. Enter opens it. On a live session its model is in memory, gone at power off"
        }
        false => "Installed for this session, held in memory until restart. Enter opens it",
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Reason {
    /// One sentence for a person.
    pub line: &'static str,
    /// Whether asking again could succeed without the machine changing.
    pub retry: bool,
}

const fn said(line: &'static str, retry: bool) -> Reason {
    Reason { line, retry }
}

pub fn reason(code: u8) -> Reason {
    match code {
        WHY_INDEX => said("The package index did not download or verify", true),
        WHY_PROGRAM_MISSING => said(
            "Its chat program, /bin/qwenchat, is not in the store yet: the store may still \
             be loading from the stick. Retry in a minute; log VFSD says how it stands",
            true,
        ),
        WHY_PROGRAM_UNREADABLE => said(
            "Its chat program, /bin/qwenchat, is in the store and could not be read; \
             the serial log (log LINUX) says why",
            true,
        ),
        WHY_NOT_PROVIDED => said("Something it needs is in no index", true),
        WHY_TOO_LARGE => said("It needs more packages than this machine allows", false),
        WHY_PACKAGE => said("A package did not download, or did not verify", true),
        WHY_NO_MIRROR => said("This system has no mirror for it", false),
        WHY_NO_KEYRING => said("This system holds no key to check it with", false),
        WHY_NO_VOLUME => said(
            "A live boot with no NONOS disk at all has nowhere to hold a model. \
             Install NONOS: Install is in the dock",
            false,
        ),
        WHY_VOLUME_LOCKED => said("The data volume is locked; unlock it, then retry", true),
        WHY_NO_NETWORK => {
            said("No network to download the model over: the chosen one is not running", true)
        }
        WHY_MODEL_UNKNOWN => said("No signed model catalogue on this system lists it", false),
        WHY_MODEL_MISMATCH => {
            said("The model did not match its pinned SHA-256, so none of it was kept", true)
        }
        WHY_MODEL_FETCH => {
            said("The model did not finish downloading; retry to go on from there", true)
        }
        WHY_NO_ROOM => said(
            "No room for the model. On a live session it is held in memory: close programs \
             to free memory and retry, or install NONOS",
            true,
        ),
        WHY_MODEL_BUSY => said("Another model download is running; retry when it ends", true),
        WHY_MODEL_UNKEPT => {
            said("This system cannot keep this model: its file names are too long", false)
        }
        WHY_MODEL_TOO_LARGE => {
            said("This machine has too little memory to run it; choose a smaller tier", false)
        }
        /*
         * Honest for both cases the kernel answers EIO in today: a disk that
         * failed, and a machine with no disk carrying NONOS at all.
         */
        WHY_VOLUME_FAILED => said(
            "The data volume could not be reached. Check the disk, then retry; \
             with no NONOS disk at all there is nowhere to hold a model",
            true,
        ),
        WHY_PARTIAL => said("Not every file could be written, so none of it was kept", true),
        WHY_NOT_INSTALLED => said("Nothing of it is installed on this system", false),
        WHY_MODEL_KEPT => said(
            "This system was built without the model fetcher, which takes a model off the \
             data volume, so its model stays",
            false,
        ),
        WHY_MODEL_DOWNLOADING => said(
            "Part of its model is still downloading; let it finish or stop it, \
             then uninstall again",
            true,
        ),
        WHY_REMOVE_PARTIAL => {
            said("Some of its files would not go; uninstall again to finish", true)
        }
        WHY_BOOT_OFFLINE => said(
            "This boot runs no network (Air-Gapped, Safe Mode or Recovery), and a model is \
             downloaded or taken off the data volume only on a boot that does",
            false,
        ),
        WHY_NO_MEMORY => said(
            "Too little free memory to hold models on this live session: install NONOS \
             to keep them on a disk",
            false,
        ),
        WHY_NYM_UNREACHABLE => said(
            "Nym is not reachable from this machine: no connection through it reached the \
             model mirror. Check the network, or choose another in Settings",
            true,
        ),
        WHY_ANYONE_UNREACHABLE => said(
            "Anyone is not reachable from this machine: no connection through it reached the \
             model mirror. Check the network, or choose another in Settings",
            true,
        ),
        WHY_MIRROR_UNREACHABLE => said(
            "The model mirror is not reachable from this machine over a direct connection. \
             Check the network, or choose another in Settings",
            true,
        ),
        WHY_NYM_NO_EXIT => said(
            "No Nym exit answered; try again, or press d to download direct (the mirror sees \
             this machine's address)",
            true,
        ),
        WHY_ANYONE_NOT_UP => said(
            "Anyone did not build a circuit within 3 minutes; retry, or press d to download \
             direct (the mirror sees this machine's address)",
            true,
        ),
        WHY_ANYONE_NO_EXIT => said(
            "No Anyone circuit reached the mirror; try again, or press d to download direct \
             (the mirror sees this machine's address)",
            true,
        ),
        _ => said(
            "The install stopped for a reason this system does not name; retry, and if it \
             stops again the serial log says why",
            true,
        ),
    }
}
