// NONOS Operating System (AGPL-3.0-or-later)
//! The controller's scripted ports.

use super::device::device;
use super::window::call;
use super::Port;

pub fn connected_ports(_xhci: u32) -> Result<Vec<Port>, i32> {
    call()?;
    Ok(device(|d| d.ports.clone()))
}
