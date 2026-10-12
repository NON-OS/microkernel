// NONOS Operating System (AGPL-3.0-or-later)
//! The engine's Io, Clock and Mem over one shared model world, and a rig:
//! devices plugged in and the controller started by the shipped code.

use std::cell::RefCell;
use std::rc::Rc;

use super::device::Device;
use super::world::{World, BUS};
use crate::env::{Clock, Io, Mem};
use crate::hc::start::start;
use crate::server::context::Ctx;

pub type Shared = Rc<RefCell<World>>;
pub struct ModelIo(pub Shared);
pub struct ModelClock(pub Shared);
pub struct ModelMem(pub Shared);
pub type Cx = Ctx<ModelIo, ModelClock, ModelMem>;

impl Io for ModelIo {
    fn r16(&mut self, off: u16) -> u16 {
        self.0.borrow_mut().read16(off)
    }
    fn w16(&mut self, off: u16, v: u16) {
        self.0.borrow_mut().write16(off, v)
    }
    fn w32(&mut self, off: u16, v: u32) {
        self.0.borrow_mut().write32(off, v)
    }
}

impl Clock for ModelClock {
    fn now_ms(&self) -> u64 {
        self.0.borrow().ms
    }
    fn idle(&self) {
        self.0.borrow_mut().tick()
    }
}

impl Mem for ModelMem {
    fn bus(&self, off: usize) -> u32 {
        BUS + off as u32
    }
    fn rd(&self, off: usize) -> u32 {
        self.0.borrow().rd32(BUS + off as u32)
    }
    fn wr(&self, off: usize, v: u32) {
        self.0.borrow_mut().wr32(BUS + off as u32, v)
    }
    fn read_bytes(&self, off: usize, out: &mut [u8]) {
        out.copy_from_slice(&self.0.borrow().mem[off..off + out.len()]);
    }
    fn write_bytes(&self, off: usize, src: &[u8]) {
        self.0.borrow_mut().mem[off..off + src.len()].copy_from_slice(src);
    }
}

/// A started controller with `devs` on ports 1 and 2.
pub fn rig(devs: [Option<Device>; 2]) -> (Cx, Shared) {
    let w = Rc::new(RefCell::new(World::default()));
    for (i, d) in devs.into_iter().enumerate() {
        if let Some(d) = d {
            w.borrow_mut().attach(i + 1, d);
        }
    }
    let mut cx = Ctx::new(ModelIo(w.clone()), ModelClock(w.clone()), ModelMem(w.clone()));
    start(&mut cx.io, &cx.clock, &cx.mem).expect("the controller starts");
    (cx, w)
}
