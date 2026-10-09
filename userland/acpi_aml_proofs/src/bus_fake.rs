// NONOS Operating System (AGPL-3.0-or-later)
//! A register bus that records every cycle and answers reads from a map, so
//! the GAS, sleep and reset proofs can check the exact accesses made.

use alloc::collections::BTreeMap;
use alloc::vec::Vec;

use crate::arch::x86_64::acpi::hw::gas::RegisterBus;
use crate::arch::x86_64::acpi::hw::reset::ResetBus;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Op {
    IoRead(u16, u8),
    IoWrite(u16, u8, u32),
    MemRead(u64, u8),
    MemWrite(u64, u8, u64),
    Pci(u8, u8, u16, u8),
    Delay(u32),
}

#[derive(Default)]
pub struct FakeBus {
    pub ops: Vec<Op>,
    pub io: BTreeMap<u16, u32>,
    pub mem: BTreeMap<u64, u64>,
}

impl FakeBus {
    pub fn writes(&self) -> Vec<Op> {
        self.ops
            .iter()
            .copied()
            .filter(|o| matches!(o, Op::IoWrite(..) | Op::MemWrite(..) | Op::Pci(..)))
            .collect()
    }

    pub fn delay_total(&self) -> u64 {
        self.ops.iter().map(|o| if let Op::Delay(u) = o { *u as u64 } else { 0 }).sum()
    }
}

impl RegisterBus for FakeBus {
    fn read_io(&mut self, port: u16, bits: u8) -> u32 {
        self.ops.push(Op::IoRead(port, bits));
        *self.io.get(&port).unwrap_or(&0)
    }
    fn write_io(&mut self, port: u16, bits: u8, value: u32) {
        self.ops.push(Op::IoWrite(port, bits, value));
    }
    fn read_mem(&mut self, phys: u64, bits: u8) -> Option<u64> {
        self.ops.push(Op::MemRead(phys, bits));
        Some(*self.mem.get(&phys).unwrap_or(&0))
    }
    fn write_mem(&mut self, phys: u64, bits: u8, value: u64) -> bool {
        self.ops.push(Op::MemWrite(phys, bits, value));
        true
    }
}

impl ResetBus for FakeBus {
    fn pci_write8(&mut self, device: u8, function: u8, offset: u16, value: u8) -> bool {
        self.ops.push(Op::Pci(device, function, offset, value));
        true
    }
    fn delay_us(&mut self, us: u32) {
        self.ops.push(Op::Delay(us));
    }
}
