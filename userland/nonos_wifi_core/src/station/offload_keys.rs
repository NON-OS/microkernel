// NONOS Operating System (AGPL-3.0-or-later)

//! A link's keys as a chip takes them (`crate::key::ChipKey`), borrowed
//! from the station that keeps them: the pairwise key, and a CCMP-128 group
//! key with the integrity group key beside it.

use super::{Ccmp, LinkStation};
use crate::key::ChipKey;
use crate::rsn::TemporalKey;

impl LinkStation {
    /// The pairwise key, as a chip takes it.
    pub fn chip_pairwise(&self) -> Option<ChipKey<'_>> {
        match &self.ccmp {
            Ccmp::Software { tk } | Ccmp::Offload { tk, .. } => Some(ChipKey::Pairwise(tk)),
            _ => None,
        }
    }

    /// The CCMP-128 group key at `id` with, under management frame
    /// protection, the integrity group key; `None` for any other cipher,
    /// which the station keeps to itself.
    pub fn chip_group(&self, id: u8) -> Option<ChipKey<'_>> {
        let TemporalKey::Ccmp128(gtk) = self.group_keys.get(usize::from(id))?.as_ref()? else {
            return None;
        };
        let igtk = self.pmf.igtk.as_ref().map(|(k, i)| (*i as u8, k));
        Some(ChipKey::Group { id, gtk, igtk })
    }

    /// The index of the group key the AP last gave.
    pub fn gtk_id(&self) -> Option<u8> {
        self.supplicant.as_ref().map(|s| s.gtk_id())
    }
}
