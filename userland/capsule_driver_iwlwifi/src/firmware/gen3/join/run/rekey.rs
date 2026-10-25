// NONOS Operating System (AGPL-3.0-or-later)

//! Keys the access point rekeyed to, put into the firmware and then wiped
//! where they waited: a pairwise key first, then a group key.

use super::super::super::region::{Clock, Region};
use super::super::fw::Fw;
use super::super::keys::KeyError;
use super::ends::Joined;
use crate::regs::Mmio;

impl Joined {
    /// Put the waiting keys into the firmware, then wipe each, whether it
    /// went in or not. `None` when none waits.
    pub fn rekey<M: Mmio, R: Region + ?Sized, C: Clock>(
        &mut self,
        fw: &mut Fw<'_, '_, M, R, C>,
    ) -> Option<Result<(), KeyError>> {
        let pairwise =
            self.link.new_pairwise_key.as_ref().map(|k| self.keys.rekey_pairwise(fw, &k.0));
        self.link.new_pairwise_key = None;
        let group = self.link.new_group_key.as_ref().map(|(id, k)| self.keys.rekey(fw, &k.0, *id));
        self.link.new_group_key = None;
        match (pairwise, group) {
            (None, None) => None,
            (p, g) => Some(p.unwrap_or(Ok(())).and(g.unwrap_or(Ok(())))),
        }
    }
}
