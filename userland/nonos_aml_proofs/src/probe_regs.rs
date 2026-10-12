// NONOS Operating System (AGPL-3.0-or-later)
//! Byte registers the probe host makes behave like hardware rather than
//! memory: the data half of an index/data pair, which reads and writes the
//! bank slot the index register last named, and a status register whose
//! bits stay set for a number of reads and then clear, as a busy bit does.

use crate::exec::RegionAccess;
use crate::probe::Probe;
use std::collections::BTreeMap;

pub enum Register {
    /// The data register of a pair whose index register is at `index`.
    Data { index: (u8, u64), bank: BTreeMap<u8, u8> },
    /// Reads `busy` set for `reads` more reads, then clear; `None` reads
    /// never clear it.
    Busy { reads: Option<u32>, busy: u8 },
}

impl Probe {
    pub(crate) fn modelled_read(&mut self, a: &RegionAccess<'_>) -> Option<u64> {
        let key = (a.space.0, a.address);
        let base = *self.bytes.get(&key).unwrap_or(&0);
        let selected = |bytes: &BTreeMap<(u8, u64), u8>, index| *bytes.get(&index).unwrap_or(&0);
        match self.regs.get_mut(&key).filter(|_| a.width == 1)? {
            Register::Data { index, bank } => {
                Some(u64::from(*bank.get(&selected(&self.bytes, *index)).unwrap_or(&0)))
            }
            Register::Busy { reads, busy } => {
                let set = match reads {
                    Some(0) => false,
                    Some(n) => {
                        *n -= 1;
                        true
                    }
                    None => true,
                };
                Some(u64::from(if set { base | *busy } else { base & !*busy }))
            }
        }
    }

    /// Whether the write went to a modelled register.
    pub(crate) fn modelled_write(&mut self, a: &RegionAccess<'_>, value: u64) -> bool {
        let key = (a.space.0, a.address);
        let index = match self.regs.get(&key) {
            Some(Register::Data { index, .. }) if a.width == 1 => *index,
            _ => return false,
        };
        let slot = *self.bytes.get(&index).unwrap_or(&0);
        if let Some(Register::Data { bank, .. }) = self.regs.get_mut(&key) {
            bank.insert(slot, value as u8);
        }
        true
    }
}
