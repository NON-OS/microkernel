// NONOS Operating System (AGPL-3.0-or-later)
//! The modelled FACS lock word and the firmware that shares it.

use crate::probe::Probe;

impl Probe {
    /// Firmware, holding the lock, lets go after `firmware_holds` looks.
    pub fn cas(&mut self, old: u32, new: u32) -> Option<u32> {
        let word = self.facs.as_mut()?;
        if *word & 2 != 0 && self.firmware_holds > 0 {
            self.firmware_holds -= 1;
            if self.firmware_holds == 0 {
                *word &= !3;
            }
        }
        if self.firmware_waits && *word == 2 {
            *word |= 1;
        }
        let found = *word;
        if found == old {
            *word = new;
        }
        Some(found)
    }
}
