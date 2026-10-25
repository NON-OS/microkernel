// NONOS Operating System (AGPL-3.0-or-later)
//! The controller's state: its registers, three root ports, the devices on
//! them, and the memory the schedule lives in, addressed by bus address.

use super::device::Device;
use super::frame;
use crate::facts::regs::{CTL_HCFS, FI_DEFAULT, HCFS_OPERATIONAL};
use crate::sched::layout::REGION;

/// Where the schedule region sits on the model's bus.
pub const BUS: u32 = 0x0080_0000;
pub const PORTS: usize = 3;

#[derive(Default)]
pub struct World {
    pub ms: u64,
    pub control: u32,
    pub status: u32,
    pub int_status: u32,
    pub int_disabled: u32,
    pub hcca: u32,
    pub ctrl_head: u32,
    pub bulk_head: u32,
    pub fm_interval: u32,
    pub periodic_start: u32,
    pub frame: u16,
    /// Global power on, then each port's status bits (facts::port) kept.
    pub powered: bool,
    pub ports: [u32; PORTS],
    pub reset_left: [u8; PORTS],
    pub devs: [Option<Device>; PORTS],
    pub mem: Vec<u8>,
    /// Firmware's SMM driver owns the controller; `smm_deaf` never yields.
    pub smm: bool,
    pub smm_deaf: bool,
    pub hcr_stuck: bool,
    pub legacy_off: bool,
    pub frames: u64,
}

impl World {
    pub fn new() -> Self {
        World { fm_interval: FI_DEFAULT, mem: vec![0; REGION], ..World::default() }
    }

    /// Plug `d` into root port `port` (1 to PORTS).
    pub fn attach(&mut self, port: usize, d: Device) {
        self.ports[port - 1] |= crate::facts::port::PORT_CSC;
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

    /// One millisecond: port resets go on, and a frame runs when the
    /// controller is operational.
    pub fn tick(&mut self) {
        self.ms += 1;
        self.port_tick();
        if self.control & CTL_HCFS == HCFS_OPERATIONAL {
            self.frames += 1;
            frame::run(self);
        }
    }
}
