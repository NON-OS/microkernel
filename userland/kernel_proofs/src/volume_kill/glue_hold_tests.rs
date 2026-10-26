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

//! A reused pid forgotten at purge (G10), and write_or_create as one hold:
//! the look, the import guard with the verb that names what happens, the
//! create if missing, and the write, all under one VOLUME.write().

use crate::proofs::proofs_8::transport_takeover::text::{code, tail};

const PURGE: &str = include_str!("../../../../src/process/exit/purge.rs");
const WOC: &str = include_str!("../../../../src/fs/blockfs_volume/write_or_create.rs");

#[test]
fn g10_purge_forgets_the_volume_rows_after_the_transport_holder() {
    let purge = concat!(
        "pub(crate) fn purge_for_new_pid(pid: crate::process::core::Pid) {",
        " super::postmortem::purge_for_new_pid(pid); super::reap_log::purge_for_new_pid(pid);",
        " crate::services::lifecycle::forget_transport_holder(pid);",
        " crate::fs::blockfs_volume::state::forget_holder(pid); }",
    );
    assert_eq!(tail(PURGE, "pub(crate) fn purge_for_new_pid("), code(purge));
}

#[test]
fn write_or_create_is_one_hold_with_the_right_verb() {
    let woc = concat!(
        "pub fn write_or_create(path: &[u8], data: &[u8]) -> Result<(), VolumeError> {",
        " let mut guard = VOLUME.write();",
        " let state = guard.as_mut().ok_or(VolumeError::NotMounted)?;",
        " let exists = match blockfs::resolve(&state.key, &state.mount, path) {",
        " Ok(_) => true, Err(BlockFsError::NotFound) => false,",
        " Err(e) => return Err(VolumeError::BlockFs(e)), };",
        " let what = if exists { \"write\" } else { \"create\" };",
        " super::import_guard::guard(&state.key, &state.mount, path, what)?;",
        " if !exists { blockfs::create_path(&state.key, &mut state.mount, path, MODE_FILE)",
        " .map_err(VolumeError::BlockFs)?; }",
        " blockfs::write_path(&state.key, &mut state.mount, path, data)",
        ".map_err(VolumeError::BlockFs) }",
    );
    assert_eq!(tail(WOC, "pub fn write_or_create("), code(woc));
}
