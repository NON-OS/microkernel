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

//! The keys the handshake produced, in the firmware's key table: the
//! pairwise key at index 0 and the group key at the index the access point
//! named (1 to 3), both for the access point's station. The table is what
//! lets the firmware decrypt received frames; transmit frames are protected
//! by the station before they are queued and sent with no firmware key
//! (`IWL_TX_FLAGS_ENCRYPT_DIS`), so the packet numbers on the air are the
//! station's own and never collide with a counter in the firmware. What was
//! installed is remembered so a disconnect removes exactly it, and a group
//! rekey to another index removes the old key once the new one is in.

use super::super::region::{Clock, Region};
use super::super::station::ids::{DATA_PATH_GROUP, SEC_KEY_CMD};
use super::super::station::key::{add_key, remove_key, KeyKind};
use super::super::station::sta::AP_STA_MASK;
use super::fw::{CommandFailed, Fw};
use crate::regs::Mmio;

/// The pairwise key's index.
pub const PAIRWISE_KEY_ID: u8 = 0;
/// Group key indices an access point may use.
const GROUP_IDS: core::ops::RangeInclusive<u8> = 1..=3;

/// What is in the firmware's key table.
#[derive(Clone, Copy, Default, PartialEq, Eq, Debug)]
pub struct Installed {
    pub pairwise: Option<bool>,
    pub group: Option<u8>,
}

/// Why a key did not go in.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum KeyError {
    /// The pairwise key's command failed.
    Pairwise(CommandFailed),
    /// The group key's command failed, or its index is not 1 to 3.
    Group(Option<CommandFailed>),
}

impl Installed {
    /// Install the pairwise key (with management frame protection `pmf`)
    /// and the group key `gtk_id`.
    pub fn install<M: Mmio, R: Region + ?Sized, C: Clock>(
        &mut self,
        fw: &mut Fw<'_, '_, M, R, C>,
        tk: &[u8; 16],
        pmf: bool,
        gtk: &[u8; 16],
        gtk_id: u8,
    ) -> Result<(), KeyError> {
        let kind = KeyKind::Pairwise { mfp: pmf };
        fw.command(DATA_PATH_GROUP, SEC_KEY_CMD, &add_key(AP_STA_MASK, PAIRWISE_KEY_ID, kind, tk))
            .map_err(KeyError::Pairwise)?;
        self.pairwise = Some(pmf);
        self.rekey(fw, gtk, gtk_id)
    }

    /// Install a group key at `id`, then remove the one it replaces if that
    /// was at another index.
    pub fn rekey<M: Mmio, R: Region + ?Sized, C: Clock>(
        &mut self,
        fw: &mut Fw<'_, '_, M, R, C>,
        gtk: &[u8; 16],
        id: u8,
    ) -> Result<(), KeyError> {
        if !GROUP_IDS.contains(&id) {
            return Err(KeyError::Group(None));
        }
        fw.command(DATA_PATH_GROUP, SEC_KEY_CMD, &add_key(AP_STA_MASK, id, KeyKind::Group, gtk))
            .map_err(|c| KeyError::Group(Some(c)))?;
        if let Some(old) = self.group.filter(|&old| old != id) {
            let _ = fw.command(DATA_PATH_GROUP, SEC_KEY_CMD, &remove_key(AP_STA_MASK, old, KeyKind::Group));
        }
        self.group = Some(id);
        Ok(())
    }

    /// Remove every key installed (best effort).
    pub fn remove<M: Mmio, R: Region + ?Sized, C: Clock>(&mut self, fw: &mut Fw<'_, '_, M, R, C>) {
        if let Some(id) = self.group.take() {
            let _ = fw.command(DATA_PATH_GROUP, SEC_KEY_CMD, &remove_key(AP_STA_MASK, id, KeyKind::Group));
        }
        if let Some(mfp) = self.pairwise.take() {
            let kind = KeyKind::Pairwise { mfp };
            let _ = fw.command(DATA_PATH_GROUP, SEC_KEY_CMD, &remove_key(AP_STA_MASK, PAIRWISE_KEY_ID, kind));
        }
    }
}
