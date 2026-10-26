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

//! Why a shipped tier's model was not brought or taken off: the data volume,
//! the download, or the machine (why.rs). A model is a dependency the model
//! fetcher brings, never a package, so none of these is NotProvided.

use super::Why;

#[allow(non_upper_case_globals)]
impl Why {
    /// No data volume to keep a model on: no disk carries NONOS at all
    /// (ENODEV). A live stick's session volume is in memory, so this is a
    /// machine with no NONOS disk. No retry mends it.
    pub const NoVolume: Self = Self(10);
    /// The data volume is locked until its passphrase is given.
    pub const VolumeLocked: Self = Self(11);
    /// No network to fetch a model over: the network the download must
    /// leave through is not running. A retry once it runs gets past it.
    pub const NoNetwork: Self = Self(12);
    /// No signed model catalogue lists the tier, or the name is in the
    /// tiers' namespace and is no shipped tier.
    pub const ModelUnknown: Self = Self(13);
    /// A model's bytes were not its signed pin; none of them was kept.
    pub const ModelMismatch: Self = Self(14);
    /// A model did not finish downloading; what came is kept for next time.
    pub const ModelFetch: Self = Self(15);
    /// The data volume has no room for the model (ENOSPC). On a live session
    /// the volume is in memory: freeing memory, then a retry, gets past it.
    pub const NoRoom: Self = Self(16);
    /// Another model download is running; one feeds the volume at a time.
    pub const ModelBusy: Self = Self(17);
    /// A model file's name is too long for the data volume to keep, so this
    /// system cannot install the tier at all; trying again changes nothing.
    pub const ModelUnkept: Self = Self(18);
    /// The tier needs more memory to run than this machine has in all, so
    /// its model was not downloaded; a smaller tier fits.
    pub const ModelTooLarge: Self = Self(19);
    /// The data volume could not be reached: the disk failed (EIO), is not
    /// ready yet (EAGAIN), or refused otherwise. A retry may get past it.
    pub const VolumeFailed: Self = Self(20);
    /// A shipped tier whose model nothing on this system can take off the
    /// data volume: it was built without the model fetcher.
    pub const ModelKept: Self = Self(23);
    /// Too little free memory to hold a live session's data volume at all
    /// (ENOMEM). A retry does not make memory.
    pub const NoMemory: Self = Self(25);
    /// A file of a tier being uninstalled is still downloading; the kernel
    /// takes nothing away under a stream being fed. A paused one is removed.
    pub const ModelDownloading: Self = Self(26);
}
