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

//! Bringing a model, or any large file, onto the data volume, verified.
//!
//! The digest it must have is the caller's: a signed capsule pins it in its
//! own code, so the capsule's measurement covers it. The disk only says
//! where the bytes wait. The file is linked under its name only after its
//! digest matched, so a bad import never becomes a file; its sealed blocks
//! stay allocated, since the volume only allocates forward.

use super::error::VolumeError;
use super::hex::{as_str, hex32};
use super::import_record::{record, recorded};
use super::import_stream::stream_in;
use super::open_machine::open_machine_volume;
use super::plan_read::read_plan;
use super::state::VOLUME;
use crate::fs::blockfs::{self, FileStream, MODE_FILE};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Imported {
    pub bytes: u64,
    pub sha256: [u8; 32],
    /// False when an earlier boot had already imported and verified it.
    pub fresh: bool,
}

/// Import the plan's file as `name`, keeping it only if its SHA-256 is `want`.
pub fn import(name: &[u8], want: &[u8; 32]) -> Result<Imported, VolumeError> {
    open_machine_volume()?;
    if let Some(bytes) = recorded(name, want)? {
        return Ok(Imported { bytes, sha256: *want, fresh: false });
    }
    let (at, bytes) = read_plan()?.import.ok_or(VolumeError::NoImport)?;
    let mut guard = VOLUME.write();
    let state = guard.as_mut().ok_or(VolumeError::NotMounted)?;
    let (key, mount) = (&state.key, &mut state.mount);
    let mut stream = FileStream::new();
    let got = stream_in(key, mount, &mut stream, at, bytes)?;
    if &got != want {
        let (g, w) = (hex32(&got), hex32(want));
        crate::log::warn!(
            "[DATA] import refused: sha256 {} is not the pinned {}",
            as_str(&g),
            as_str(&w)
        );
        return Err(VolumeError::DigestMismatch);
    }
    let lba = blockfs::create_path(key, mount, name, MODE_FILE).map_err(VolumeError::BlockFs)?;
    let mut node = blockfs::read_node(key, lba).map_err(VolumeError::BlockFs)?;
    stream.finish(key, mount, lba, &mut node).map_err(VolumeError::BlockFs)?;
    record(key, mount, name, want)?;
    crate::log::info!("[DATA] imported {} bytes, sha256 {}", bytes, as_str(&hex32(want)));
    Ok(Imported { bytes, sha256: *want, fresh: true })
}
