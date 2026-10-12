// NONOS Operating System (AGPL-3.0-or-later)
//! A host that records what the interpreter asked of it and models what a
//! field reaches: spaces, transactions, pins, tables, the FACS lock word.

use crate::exec::{PciPath, RegionAccess};
use std::collections::BTreeMap;

/// A transaction as the handler saw it: space, address, access, connection,
/// data in.
pub type Transaction = (u8, u64, [u8; 3], Option<Vec<u8>>, Vec<u8>);

#[derive(Default)]
pub struct Probe {
    pub slept_ms: Vec<u64>,
    pub stalled_us: Vec<u64>,
    pub ticks: u64,
    pub debug: Vec<String>,
    pub fatal: Vec<(u8, u32, u64)>,
    /// Bytes by (space, address).
    pub bytes: BTreeMap<(u8, u64), u8>,
    /// Every register access: space, address, width, and the value written.
    pub log: Vec<(u8, u64, u32, Option<u64>)>,
    pub pci: Vec<PciPath>,
    pub refuse: bool,
    pub transactions: Vec<Transaction>,
    pub pins: BTreeMap<u32, u64>,
    pub tables: Vec<Vec<u8>>,
    /// The FACS lock word, if any; firmware lets go after this many looks.
    pub facs: Option<u32>,
    pub firmware_holds: u32,
    /// Firmware asks for the lock while the capsule holds it.
    pub firmware_waits: bool,
    pub released: u32,
}

impl Probe {
    pub fn note(&mut self, a: &RegionAccess<'_>, value: Option<u64>) {
        self.log.push((a.space.0, a.address, a.width, value));
        if let Some(p) = a.pci {
            self.pci.push(p.clone());
        }
    }

    pub fn read(&mut self, a: &RegionAccess<'_>) -> Option<u64> {
        self.note(a, None);
        let byte = |i: u64| *self.bytes.get(&(a.space.0, a.address + i)).unwrap_or(&0) as u64;
        let v = (0..a.width as u64).rev().fold(0u64, |v, i| (v << 8) | byte(i));
        (!self.refuse).then_some(v)
    }

    pub fn write(&mut self, a: &RegionAccess<'_>, value: u64) -> bool {
        self.note(a, Some(value));
        for i in 0..a.width as u64 {
            self.bytes.insert((a.space.0, a.address + i), (value >> (8 * i)) as u8);
        }
        !self.refuse
    }
}
