// NONOS Operating System (AGPL-3.0-or-later)
//! The controller's state: its registers, three root ports with the speed
//! of the device on each, and the memory the schedule lives in.

use super::device::Device;
use super::frame;
use crate::facts::regs::{CMD_RS, STS_HALTED};
use crate::sched::layout::REGION;

/// Where the schedule region sits on the model's bus.
pub const BUS: u32 = 0x0080_0000;
/// The operational registers' base (CAPLENGTH).
pub const CAPLEN: u32 = 0x20;
pub const PORTS: usize = 3;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum Speed {
    #[default]
    High,
    Full,
    Low,
}

#[derive(Default)]
pub struct World {
    pub ms: u64,
    pub cmd: u32,
    pub sts: u32,
    pub frindex: u32,
    pub periodic: u32,
    pub async_addr: u32,
    pub configflag: u32,
    /// Each port's written bits (enable, reset, power, owner, changes).
    pub ports: [u32; PORTS],
    pub devs: [Option<Device>; PORTS],
    pub speed: [Speed; PORTS],
    /// Port resets begun, by port, and ports handed to the companion.
    pub resets: [u32; PORTS],
    pub handed: [u32; PORTS],
    pub mem: Vec<u8>,
    pub doorbell: bool,
    pub hcreset_stuck: bool,
    pub frames: u64,
}

impl World {
    pub fn new() -> Self {
        World { sts: STS_HALTED, mem: vec![0; REGION], ..World::default() }
    }

    /// Plug `d` of `speed` into root port `port` (1 to PORTS).
    pub fn attach(&mut self, port: usize, d: Device, speed: Speed) {
        self.ports[port - 1] |= crate::facts::port::PORT_CSC;
        (self.devs[port - 1], self.speed[port - 1]) = (Some(d), speed);
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
        if self.cmd & CMD_RS != 0 && self.sts & STS_HALTED == 0 {
            self.frames += 1;
            frame::run(self);
        }
    }
}
