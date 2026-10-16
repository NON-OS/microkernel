// NONOS Operating System (AGPL-3.0-or-later)

//! The handshake's keys wiped when the supplicant goes: the PMK, the PTK,
//! the GTK and the IGTK, and the SNonce they were derived with.

use super::fields::Supplicant;
use crate::wipe::wipe;

impl Drop for Supplicant {
    fn drop(&mut self) {
        wipe(&mut self.pmk);
        wipe(&mut self.snonce);
        wipe(&mut self.ptk);
        wipe(&mut self.gtk);
        wipe(&mut self.igtk);
    }
}
