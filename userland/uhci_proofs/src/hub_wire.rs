// NONOS Operating System (AGPL-3.0-or-later)
//! What the hub lane sends its requests through: one modelled controller
//! by its own dispatch, or the host front over several (server/many),
//! every controller's clock moving together.

use nonos_usb_hub::RootHost;

use crate::env::Clock;
use crate::hub_lane::Lane;
use crate::model::{call, call_host, Answer, Cx};
use crate::server::many::Front;
use crate::wire::ops::OP_ADDRESS_DEVICE;

pub trait Wire {
    fn call(&mut self, op: u16, body: &[u8]) -> Answer;
    fn idle(&mut self);
    fn now(&mut self) -> u64;
}

impl Wire for Cx {
    fn call(&mut self, op: u16, body: &[u8]) -> Answer {
        call(self, op, body)
    }
    fn idle(&mut self) {
        self.clock.idle()
    }
    fn now(&mut self) -> u64 {
        self.clock.now_ms()
    }
}

impl Wire for Front<Cx> {
    fn call(&mut self, op: u16, body: &[u8]) -> Answer {
        call_host(self, op, body)
    }
    fn idle(&mut self) {
        self.hcs.iter().flatten().for_each(|cx| cx.clock.idle());
    }
    fn now(&mut self) -> u64 {
        self.hcs.iter().flatten().next().map_or(0, |cx| cx.clock.now_ms())
    }
}

/// A wire status as a result.
pub fn done(status: i32) -> Result<(), i32> {
    if status == 0 {
        Ok(())
    } else {
        Err(status)
    }
}

/// The lane's root port addresses, as driver.usb_hub0 makes them.
impl<W: Wire> RootHost for Lane<'_, W> {
    fn address_root(&mut self, slot: u8, port: u8) -> Result<u8, i32> {
        let r = self.cx.call(OP_ADDRESS_DEVICE, &[slot, port]);
        done(r.status).map(|_| r.data[2])
    }
}
