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
 * Uninstalling a shipped tier: its program is the image's and stays; its
 * model files are taken off the data volume by the model fetcher's `remove`,
 * which holds the right the kernel asks for that. Each way it can stop is
 * the uninstall's exit code (`Why`). Pure, so model_fetch_proofs holds it.
 */

use super::fetch_exit::{BUSY, DONE, LOCKED, NO_MEMORY, NO_VOLUME, USAGE, VOLUME_FAILED};
use super::why::Why;

/* MkToolRun's errnos, negative. */
const NOT_BUILT: i64 = -2;
const RUNNING: i64 = -16;
const NETDOWN: i64 = -100;

/* Why the fetcher did not start to remove, from MkToolRun's errno. */
pub fn remove_not_started(rc: i64) -> Why {
    match rc {
        /* Built without the fetcher: nothing on this system can remove a model. */
        NOT_BUILT => Why::ModelKept,
        /* One fetcher runs at a time; a download, or another tier's removal. */
        RUNNING => Why::ModelBusy,
        /*
         * The fetcher holds the network, so a boot that runs none never
         * starts it: nothing was tried, and nothing on this boot will be.
         */
        NETDOWN => Why::BootOffline,
        _ => Why::RemovePartial,
    }
}

/* The fetcher's exit status for `remove <tier>` as the uninstall's reason. */
pub fn removed(status: i64) -> Result<(), Why> {
    let Ok(status) = i32::try_from(status) else { return Err(Why::RemovePartial) };
    match status {
        DONE => Ok(()),
        /* No disk carries NONOS, so no volume, so no model on one to take away. */
        NO_VOLUME => Ok(()),
        /* A stream is still being fed to a file: a download under way. */
        BUSY => Err(Why::ModelDownloading),
        LOCKED => Err(Why::VolumeLocked),
        VOLUME_FAILED => Err(Why::VolumeFailed),
        NO_MEMORY => Err(Why::NoMemory),
        USAGE => Err(Why::ModelUnknown),
        _ => Err(Why::RemovePartial),
    }
}
