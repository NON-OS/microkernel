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

//! What to do after each way an install or an update can stop, and what
//! an update that stopped left of the disk.

pub const RETRY_TABLE: &str = "Choose this disk again to start over, or another disk. Nothing else changed, and the drive you started from still boots.";
pub const RETRY_READBACK: &str =
    "Install onto this disk again. If the read-back fails twice, the disk is failing: use another.";
pub const RETRY_UPDATE: &str = "Choose this disk again and update it once more to finish. Until then your data stays as it was, and NØNOS does not start from this disk.";
pub const UPDATE_STOPPED: &str =
    "The update stopped before the new system was complete. Your data volume was not touched.";
pub const UPDATE_READBACK: &str = "Your data volume was not touched, but the new system did not read back as written. Do not boot it.";
pub const RETRY_UPDATE_READBACK: &str =
    "Update this disk again. If the read-back fails twice, the disk is failing.";
pub const RETRY_OTHER: &str =
    "Choose a disk again. If it repeats, photograph this screen and report it.";
