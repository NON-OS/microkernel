// NONOS Operating System (AGPL-3.0-or-later)

//! The station's keys wiped where they lie before they are forgotten: an
//! `Option` set to `None` or a mode set to `Hardware` leaves the old bytes.

use super::{Ccmp, LinkStation};
use crate::wipe::wipe;

impl LinkStation {
    /// Wipe the group keys and the software pairwise key, then forget them.
    pub(super) fn forget_keys(&mut self) {
        for key in self.group_keys.iter_mut().flatten() {
            wipe(key);
        }
        self.group_keys = [None; 4];
        if let Ccmp::Software { tk } | Ccmp::SoftwareTxHwRx { tk } = &mut self.ccmp {
            wipe(tk);
        }
        self.ccmp = Ccmp::Hardware;
    }
}
