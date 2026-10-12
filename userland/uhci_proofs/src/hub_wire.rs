// NONOS Operating System (AGPL-3.0-or-later)
//! What the hub lane sends its requests through: one modelled controller
//! by its own dispatch, or the host front over several (server/many),
//! every controller's clock moving together.

use crate::env::Clock;
use crate::model::{call, call_host, Answer, Cx};
use crate::server::many::Front;

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
