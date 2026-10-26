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

// The kernel's data volume (src/fs/blockfs_volume), its keying, its live
// boot's volume and the switch when keeping turns on, mounted as the
// `blockfs_volume` module of mount_fs.rs's `fs`. Its proofs sit inside it,
// where its `pub(super)` calls are in reach. keep_switch.rs, which
// session.rs mounts, finds its two neighbours standing in below.
#[path = "../../../src/fs/blockfs_volume/error.rs"]
pub mod error;
#[path = "../../../src/fs/blockfs_volume/format_volume.rs"]
pub mod format_volume;
#[path = "../../../src/fs/blockfs_volume/key_header.rs"]
mod key_header;
#[path = "../../../src/fs/blockfs_volume/key_header_io.rs"]
mod key_header_io;
#[path = "../../../src/fs/blockfs_volume/key_seal.rs"]
mod key_seal;
#[path = "../../../src/fs/blockfs_volume/mount_or_format.rs"]
mod mount_or_format;
#[path = "../../../src/fs/blockfs_volume/opened.rs"]
mod opened;
#[path = "../../../src/fs/blockfs_volume/passphrase_create.rs"]
mod passphrase_create;
#[path = "../../../src/fs/blockfs_volume/passphrase_key.rs"]
mod passphrase_key;
#[path = "../../../src/fs/blockfs_volume/plan_types.rs"]
pub mod plan_types;
#[path = "../../../src/fs/blockfs_volume/ring_blank.rs"]
mod ring_blank;
#[path = "../../../src/fs/blockfs_volume/say.rs"]
mod say;
#[path = "../../../src/fs/blockfs_volume/session.rs"]
mod session;
#[path = "../../../src/fs/blockfs_volume/state.rs"]
mod state;
pub use session::keeping_on;
#[path = "volume_tests/mod.rs"]
mod tests;

/// The stream lock: no import is ever in flight here.
mod import_feed {
    pub(in crate::fs) fn with_streams<R>(f: impl FnOnce() -> (R, bool)) -> R {
        f().0
    }
}

/// The names a dropped RAM volume held: none are counted here.
mod list {
    use super::{error::VolumeError, state::VolumeState};
    pub(in crate::fs) fn names_in(_: &VolumeState, _: &[u8]) -> Result<usize, VolumeError> {
        Ok(0)
    }
}
