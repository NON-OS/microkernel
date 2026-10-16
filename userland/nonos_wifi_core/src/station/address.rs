// NONOS Operating System (AGPL-3.0-or-later)

//! The station's own address, changed only between associations.

use super::LinkStation;
use crate::dot11::header::MacAddr;

impl LinkStation {
    /// Take `mac` as the address the next association runs under. Refused
    /// (false, nothing changed) while associated: a session keeps the
    /// address it began with.
    pub fn readdress(&mut self, mac: MacAddr) -> bool {
        if self.associated {
            return false;
        }
        self.our_mac = mac;
        true
    }
}
