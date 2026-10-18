// NONOS Operating System (AGPL-3.0-or-later)
//! Two register models the probe host applies at any access width: a
//! window whose bytes depend on the bank a select register holds, as a
//! BankField reaches, and a data register that reads and writes the bytes
//! at the offset an index register holds, as an IndexField reaches.

use crate::exec::RegionAccess;
use crate::probe::Probe;
use std::collections::BTreeMap;

pub struct Banked {
    pub space: u8,
    /// The window, `lo..hi`, and the one-byte select register.
    pub lo: u64,
    pub hi: u64,
    pub select: u64,
    /// Bytes by (bank, address).
    pub bytes: BTreeMap<(u8, u64), u8>,
}

pub struct Window {
    pub space: u8,
    /// The index register and its width in bytes, and the data register.
    pub index: u64,
    pub index_len: u64,
    pub data: u64,
    /// Bytes by offset.
    pub bytes: BTreeMap<u64, u8>,
}

impl Probe {
    /// The byte at `i` of this access in a modelled register, if it is one.
    fn slot(&self, a: &RegionAccess<'_>, i: u64) -> Option<(bool, u64, u8)> {
        let at = a.address + i;
        if let Some(b) = self.banks.as_ref().filter(|b| b.space == a.space.0) {
            if (b.lo..b.hi).contains(&a.address) {
                let bank = *self.bytes.get(&(b.space, b.select)).unwrap_or(&0);
                return Some((true, at, bank));
            }
        }
        let w = self.window.as_ref().filter(|w| w.space == a.space.0 && w.data == a.address)?;
        let read = |k: u64| u64::from(*self.bytes.get(&(w.space, w.index + k)).unwrap_or(&0));
        let index = (0..w.index_len).rev().fold(0, |v, k| v << 8 | read(k));
        Some((false, index + i, 0))
    }

    fn cell(&mut self, banked: bool, key: u64, bank: u8) -> &mut u8 {
        match banked {
            true => self.banks.as_mut().unwrap().bytes.entry((bank, key)).or_insert(0),
            false => self.window.as_mut().unwrap().bytes.entry(key).or_insert(0),
        }
    }

    pub(crate) fn window_read(&mut self, a: &RegionAccess<'_>) -> Option<u64> {
        self.slot(a, 0)?;
        let bytes: Vec<u8> = (0..u64::from(a.width))
            .map(|i| self.slot(a, i).map(|(b, k, n)| *self.cell(b, k, n)).unwrap_or(0))
            .collect();
        Some(bytes.iter().rev().fold(0, |v, x| v << 8 | u64::from(*x)))
    }

    pub(crate) fn window_write(&mut self, a: &RegionAccess<'_>, value: u64) -> bool {
        for i in 0..u64::from(a.width) {
            let Some((b, k, n)) = self.slot(a, i) else { return false };
            *self.cell(b, k, n) = (value >> (8 * i)) as u8;
        }
        true
    }
}
