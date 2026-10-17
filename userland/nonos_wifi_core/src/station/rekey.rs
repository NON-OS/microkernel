// NONOS Operating System (AGPL-3.0-or-later)

//! A pairwise rekey taken: the station's own copy of the key, where it runs
//! CCMP in software, replaced in place, with its packet numbers and replay
//! counters started afresh (IEEE Std 802.11-2020, 12.5.3.4.4).

use super::mode::Ccmp;
use super::LinkStation;

impl LinkStation {
    /// Take `tk` as the pairwise key from here on. A chip that runs CCMP in
    /// hardware takes it from the driver (`Rx::Handshake::pairwise_key`).
    pub(super) fn set_pairwise_key(&mut self, tk: [u8; 16]) {
        match &mut self.ccmp {
            Ccmp::Software { tk: old } | Ccmp::SoftwareTxHwRx { tk: old } => *old = tk,
            Ccmp::Hardware => {}
        }
        self.pn = 0;
        self.replay.reset_unicast();
    }
}
