// NONOS Operating System (AGPL-3.0-or-later)

//! A group key the access point rekeyed to, put into the firmware and then
//! wiped where it waited.

use super::super::super::region::{Clock, Region};
use super::super::fw::Fw;
use super::super::keys::KeyError;
use super::ends::Joined;
use crate::regs::Mmio;

impl Joined {
    /// Put the waiting group key into the firmware, then wipe it, whether it
    /// went in or not. `None` when none waits.
    pub fn rekey<M: Mmio, R: Region + ?Sized, C: Clock>(
        &mut self,
        fw: &mut Fw<'_, '_, M, R, C>,
    ) -> Option<Result<(), KeyError>> {
        let (id, key) = self.link.new_group_key.as_ref()?;
        let done = self.keys.rekey(fw, &key.0, *id);
        self.link.new_group_key = None;
        Some(done)
    }
}
