// NONOS Operating System (AGPL-3.0-or-later)
//! The scripted reader behind a host controller that can lie about counts:
//! a command taken only in part, or more bytes said to have come than the
//! transfer was armed for. Everything else passes to the script.

use nonos_usbnet::mock::MockBus;
use nonos_usbnet::{Bus, Pipes, Setup};

pub struct Lying {
    pub bus: MockBus,
    /// The bytes of a command the device says it took.
    pub took: Option<usize>,
    /// Says each transfer in came whole, though cut to what was armed.
    pub past: bool,
}

impl Bus for Lying {
    fn control_in(&mut self, setup: Setup, out: &mut [u8]) -> Result<usize, i32> {
        self.bus.control_in(setup, out)
    }
    fn control_out(&mut self, setup: Setup, data: &[u8]) -> Result<(), i32> {
        self.bus.control_out(setup, data)
    }
    fn configure_bulk(&mut self, pipes: &Pipes) -> Result<(), i32> {
        self.bus.configure_bulk(pipes)
    }
    fn bulk_out(&mut self, data: &[u8]) -> Result<usize, i32> {
        let n = self.bus.bulk_out(data)?;
        Ok(self.took.unwrap_or(n))
    }
    fn bulk_in_poll(&mut self, out: &mut [u8]) -> Result<Option<usize>, i32> {
        let whole = self.bus.0.borrow().bulk_in.front().and_then(|t| t.as_ref().ok().cloned());
        let n = self.bus.bulk_in_poll(out)?;
        match (self.past, whole) {
            (true, Some(Some(m))) => Ok(n.map(|n| n.max(m.len()))),
            _ => Ok(n),
        }
    }
    fn reset_bulk(&mut self, dir_in: bool) -> Result<(), i32> {
        self.bus.reset_bulk(dir_in)
    }
}
