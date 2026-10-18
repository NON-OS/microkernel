// NONOS Operating System (AGPL-3.0-or-later)
//! The controller's registration and its ports: one controller, unless the
//! test says none has registered, and the scripted ports.

use super::device::{device, setup};
use super::window::call;
use super::Port;

pub fn lookup() -> Option<u32> {
    setup(|d| {
        d.lookups += 1;
        (!d.unregistered).then_some(1)
    })
}

pub fn connected_ports(_xhci: u32) -> Result<Vec<Port>, i32> {
    call()?;
    Ok(device(|d| d.ports.clone()))
}
