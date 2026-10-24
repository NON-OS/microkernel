// NONOS Operating System (AGPL-3.0-or-later)
//! A card that lies about its state: the R1 to one command index reports
//! the state a test chose, CURRENT_STATE in bits 12:9.

use super::sim::Sim;

impl Sim {
    pub(super) fn lied(&self, index: u8, r: u32) -> u32 {
        match self.lie {
            Some((i, st)) if i == index => r & !(0xf << 9) | st << 9,
            _ => r,
        }
    }
}
