// NONOS Operating System (AGPL-3.0-or-later)
//! The host's registers, its clock and what the driver wrote.

use super::device::Device;

#[derive(Default)]
pub struct Model {
    pub sts: u8,
    pub cnt: u8,
    pub cmd: u8,
    pub add: u8,
    pub dat0: u8,
    pub dat1: u8,
    pub aux: u8,
    pub blk: [u8; 32],
    pub idx: usize,
    /// The host has the 32-byte buffer (ICH4 and later).
    pub e32b: bool,
    pub devices: Vec<Device>,
    /// Status reads the host stays busy for after a start.
    pub busy_reads: u32,
    /// The host never finishes until killed.
    pub stuck: bool,
    /// Error bits the next transaction ends with in place of INTR.
    pub force: Option<u8>,
    /// Status reads after BUSY clears before the result bits show.
    pub lag: u32,
    pub writes: Vec<(u16, u8)>,
    pub now: u64,
    pub deadline: u64,
    pub(super) pending: u32,
    pub(super) result: Option<u8>,
}

impl Model {
    pub fn new(devices: Vec<Device>) -> Self {
        Self { e32b: true, devices, ..Self::default() }
    }

    pub fn wrote(&self, off: u16) -> Vec<u8> {
        self.writes.iter().filter(|(o, _)| *o == off).map(|(_, v)| *v).collect()
    }
}
