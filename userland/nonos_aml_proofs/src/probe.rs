// NONOS Operating System (AGPL-3.0-or-later)
//! A host that records what the interpreter asked of it.

use crate::exec::Host;
use crate::object::Object;

#[derive(Default)]
pub struct Probe {
    pub slept_ms: Vec<u64>,
    pub stalled_us: Vec<u64>,
    pub ticks: u64,
    pub debug: Vec<String>,
    pub fatal: Vec<(u8, u32, u64)>,
}

impl Host for Probe {
    fn sleep(&mut self, ms: u64) {
        self.slept_ms.push(ms);
    }
    fn stall(&mut self, us: u64) {
        self.stalled_us.push(us);
    }
    fn timer(&mut self) -> u64 {
        self.ticks += 10;
        self.ticks
    }
    fn debug(&mut self, value: &Object) {
        self.debug.push(format!("{value:?}"));
    }
    fn fatal(&mut self, kind: u8, code: u32, arg: u64) {
        self.fatal.push((kind, code, arg));
    }
}
