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

//! Why an install stopped, as the installer's exit code, so the system and
//! the store can say more than that it failed. The log line before the exit
//! says which package and which check.

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Why {
    /// The index or database did not fetch, or its signature did not verify.
    Index = 2,
    /// A package, or something it depends on, is in no index.
    NotProvided = 3,
    /// The closure passes the configured package limit.
    TooLarge = 4,
    /// A package did not download, or did not match its pin, checksum or
    /// signature.
    Package = 5,
    /// The image was built without a mirror for this family.
    NoMirror = 8,
    /// The image was built without a keyring for this family.
    NoKeyring = 9,
    /*
     * A shipped tier's model is a dependency the model fetcher brings, never
     * a package, so its refusals are its own and none of them is NotProvided.
     */
    /// No data volume to keep a model on: no disk carries NONOS at all
    /// (ENODEV). A live stick's session volume is in memory, so this is a
    /// machine with no NONOS disk. No retry mends it.
    NoVolume = 10,
    /// The data volume is locked until its passphrase is given.
    VolumeLocked = 11,
    /// No network to fetch a model over: the network the download must
    /// leave through is not running. A retry once it runs gets past it.
    NoNetwork = 12,
    /// No signed model catalogue lists the tier, or the name is in the
    /// tiers' namespace and is no shipped tier.
    ModelUnknown = 13,
    /// A model's bytes were not its signed pin; none of them was kept.
    ModelMismatch = 14,
    /// A model did not finish downloading; what came is kept for next time.
    ModelFetch = 15,
    /// The data volume has no room for the model (ENOSPC). On a live session
    /// the volume is in memory: freeing memory, then a retry, gets past it.
    NoRoom = 16,
    /// Another model download is running; one feeds the volume at a time.
    ModelBusy = 17,
    /// A model file's name is too long for the data volume to keep, so this
    /// system cannot install the tier at all; trying again changes nothing.
    ModelUnkept = 18,
    /// The tier needs more memory to run than this machine has in all, so
    /// its model was not downloaded; a smaller tier fits.
    ModelTooLarge = 19,
    /// The data volume could not be reached: the disk failed (EIO), is not
    /// ready yet (EAGAIN), or refused otherwise. A retry may get past it.
    VolumeFailed = 20,
    /// A package did not land whole: a file the store would not take, or
    /// links the table could not hold. What was written of it is gone.
    Partial = 21,
    /*
     * An uninstall's own refusals, the exit code of an `uninstall` run.
     */
    /// No install of the package is recorded on this system.
    NotInstalled = 22,
    /// A shipped tier whose model nothing on this system can take off the
    /// data volume: it was built without the model fetcher.
    ModelKept = 23,
    /// Not every file or link of it would go; what is left is still
    /// recorded, so asking again finishes the job.
    RemovePartial = 24,
    /// Too little free memory to hold a live session's data volume at all
    /// (ENOMEM). A retry does not make memory.
    NoMemory = 25,
    /// A file of a tier being uninstalled is still downloading; the kernel
    /// takes nothing away under a stream being fed. A paused one is removed.
    ModelDownloading = 26,
    /// This boot runs no network at all (Air-Gapped, Safe Mode or
    /// Recovery), and the model fetcher, which brings a model onto the data
    /// volume and takes one off it, starts only on a boot that does. No
    /// retry on this boot changes that; an install's or an uninstall's.
    BootOffline = 27,
    /// The chosen network runs and no connection through it reached a
    /// model mirror before a byte came: Nym, Anyone, or the mirror itself
    /// over a direct connection is not reachable from this machine.
    NymUnreachable = 28,
    AnyoneUnreachable = 29,
    MirrorUnreachable = 30,
    /// Through Nym, or Anyone, every try in a row found no exit that answered.
    NymNoExit = 31,
    AnyoneNoExit = 32,
    /// The Anyone network, which downloads installs, did not come up in time.
    AnyoneNotUp = 33,
    /// A shipped tier's program (the chat window, /bin/qwenchat) is not in
    /// this system's store, so the tier cannot be installed from it. Said
    /// apart from NotProvided, which reads as a package index that lacks
    /// something: the market's index was whole, and the store was not.
    ProgramMissing = 34,
    /// The store holds the program and it could not be read; the serial
    /// line before the exit names the store's reason.
    ProgramUnreadable = 35,
}

impl Why {
    pub fn code(self) -> i32 {
        self as i32
    }
}
