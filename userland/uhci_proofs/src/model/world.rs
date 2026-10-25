// NONOS Operating System (AGPL-3.0-or-later)
//! The controller's state: its registers, two root ports, the devices on
//! them, and the memory the schedule lives in, addressed by bus address.

use super::device::Device;
use super::frame;
use crate::facts::io::{CMD_RS, PORT_CCS, PORT_CSC, STS_HCH};
use crate::sched::layout::REGION;

/// Where the schedule region sits on the model's bus.
pub const BUS: u32 = 0x0080_0000;

pub struct World {
    pub ms: u64,
    pub cmd: u16,
    pub sts: u16,
    pub intr: u16,
    pub frnum: u16,
    pub flbase: u32,
    pub portsc: [u16; 2],
    pub devs: [Option<Device>; 2],
    pub mem: Vec<u8>,
    /// A host reset that never clears.
    pub hcreset_stuck: bool,
    pub frames: u64,
}

impl Default for World {
    fn default() -> Self {
        World {
            ms: 0,
            cmd: 0,
            sts: STS_HCH,
            intr: 0,
            frnum: 0,
            flbase: 0,
            portsc: [0; 2],
            devs: [None, None],
            mem: vec![0; REGION],
            hcreset_stuck: false,
            frames: 0,
        }
    }
}

impl World {
    /// Plug `d` into root port `port` (1 or 2).
    pub fn attach(&mut self, port: usize, d: Device) {
        self.portsc[port - 1] |= PORT_CCS | PORT_CSC;
        self.devs[port - 1] = Some(d);
    }

    pub fn rd32(&self, bus: u32) -> u32 {
        let at = (bus - BUS) as usize;
        u32::from_le_bytes(self.mem[at..at + 4].try_into().unwrap())
    }

    pub fn wr32(&mut self, bus: u32, v: u32) {
        let at = (bus - BUS) as usize;
        self.mem[at..at + 4].copy_from_slice(&v.to_le_bytes());
    }

    /// One millisecond: a frame, when the controller runs.
    pub fn tick(&mut self) {
        self.ms += 1;
        if self.cmd & CMD_RS != 0 && self.sts & STS_HCH == 0 {
            self.frames += 1;
            frame::run(self);
        }
    }
}
