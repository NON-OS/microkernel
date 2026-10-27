// NONOS Operating System (AGPL-3.0-or-later)
//! The streaming alternate and its endpoint as one step: a camera left on
//! a streaming alternate whose endpoint was never configured would hold
//! its bandwidth with nothing reading it.

use nonos_usbnet::bus::{Bus, Pipes};
use nonos_usbnet::setup::Setup;

use crate::alt::select;

/// The control requests sent, and nothing else answered.
#[derive(Default)]
struct Seen(Vec<Setup>);

impl Bus for Seen {
    fn control_in(&mut self, _: Setup, _: &mut [u8]) -> Result<usize, i32> {
        Err(-5)
    }
    fn control_out(&mut self, setup: Setup, _: &[u8]) -> Result<(), i32> {
        self.0.push(setup);
        Ok(())
    }
    fn configure_bulk(&mut self, _: &Pipes) -> Result<(), i32> {
        Err(-5)
    }
    fn bulk_out(&mut self, _: &[u8]) -> Result<usize, i32> {
        Err(-5)
    }
    fn bulk_in_poll(&mut self, _: &mut [u8]) -> Result<Option<usize>, i32> {
        Err(-5)
    }
    fn reset_bulk(&mut self, _: bool) -> Result<(), i32> {
        Err(-5)
    }
}

#[test]
fn an_endpoint_the_host_refuses_puts_the_camera_back_on_alternate_0() {
    let mut bus = Seen::default();
    assert_eq!(select(&mut bus, 1, 5, |_| Err(-28)), Err(-28), "the refusal answered");
    let back = [Setup::set_interface(1, 5), Setup::set_interface(1, 0)];
    assert_eq!(bus.0, back, "alternate 5, then 0 again");
}

#[test]
fn a_configured_endpoint_keeps_its_alternate() {
    let mut bus = Seen::default();
    let mut ran = false;
    let configured = |_: &mut Seen| {
        ran = true;
        Ok(())
    };
    assert_eq!(select(&mut bus, 1, 5, configured), Ok(()));
    assert!(ran);
    assert_eq!(bus.0, [Setup::set_interface(1, 5)]);
}
