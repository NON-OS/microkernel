// NONOS Operating System (AGPL-3.0-or-later)

//! A rekeyed pairwise key into the firmware's key table at index 0: the old
//! one removed, then the new one added, as Linux replaces a key.

use super::super::region::{Clock, Region};
use super::super::station::ids::{DATA_PATH_GROUP, SEC_KEY_CMD};
use super::super::station::key::{add_key, remove_key, KeyKind};
use super::super::station::sta::AP_STA_MASK;
use super::fw::Fw;
use super::keys::{Installed, KeyError, PAIRWISE_KEY_ID};
use crate::regs::Mmio;

impl Installed {
    /// Put `tk` in place of the pairwise key, with the protection it was
    /// installed with.
    pub fn rekey_pairwise<M: Mmio, R: Region + ?Sized, C: Clock>(
        &mut self,
        fw: &mut Fw<'_, '_, M, R, C>,
        tk: &[u8; 16],
    ) -> Result<(), KeyError> {
        let mfp = self.pairwise.unwrap_or(false);
        let kind = KeyKind::Pairwise { mfp };
        let _ = fw.command(
            DATA_PATH_GROUP,
            SEC_KEY_CMD,
            &remove_key(AP_STA_MASK, PAIRWISE_KEY_ID, kind),
        );
        fw.command(DATA_PATH_GROUP, SEC_KEY_CMD, &add_key(AP_STA_MASK, PAIRWISE_KEY_ID, kind, tk))
            .map_err(KeyError::Pairwise)?;
        self.pairwise = Some(mfp);
        Ok(())
    }
}
