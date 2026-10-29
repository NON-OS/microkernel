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

//! The machine volume's open, under the volume's write hold: the window,
//! the key header, the key the TPM gives (key_pick.rs), and the mount,
//! installed into the held volume. Nothing here takes the volume again.

use super::super::error::VolumeError;
use super::super::handover::opened;
use super::super::key_header::Keyed;
use super::super::key_header_io::read_key_header;
use super::super::key_pick::{fresh, legacy};
use super::super::mount_or_format::mount_or_format;
use super::super::opened::install;
use super::super::plan_types::Plan;
use super::super::ring_blank::ring_blank;
use super::super::state::VolumeState;

pub(in super::super) fn open_into(
    v: &mut Option<VolumeState>,
    plan: &Plan,
) -> Result<(), VolumeError> {
    crate::fs::cryptoblock::set_window(plan.volume_base, plan.volume_sectors)
        .map_err(VolumeError::Window)?;
    // The key by what the header says (key_pick.rs): a blank volume with no
    // header is formatted with a wrapped key, an old one opens as it did.
    let (key, keyed) = match read_key_header()? {
        Some(Keyed::Passphrase(_)) => {
            crate::log::warn!("[DATA] the volume is keyed by a passphrase; it waits for one");
            return Err(VolumeError::NeedsPassphrase);
        }
        Some(Keyed::Approved(wrapped)) => opened(&wrapped)?,
        None if ring_blank(plan.volume_base)? => fresh()?,
        Some(Keyed::Tpm) | None => (legacy()?, Keyed::Tpm),
    };
    let mount = mount_or_format(&key, plan, &keyed)?;
    install(v, plan, key, mount);
    Ok(())
}
