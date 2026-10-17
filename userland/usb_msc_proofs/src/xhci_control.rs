// NONOS Operating System (AGPL-3.0-or-later)
//! Control requests and pipe resets: recorded, and they succeed.

use super::DEVICE;

pub fn control_no_data(
    _xhci: u32,
    _slot: u8,
    request: (u8, u8),
    value: u16,
    index: u16,
) -> Result<(), i32> {
    DEVICE.with(|d| d.borrow_mut().controls.push((request.0, request.1, value, index)));
    Ok(())
}

pub fn control_in(
    xhci: u32,
    slot: u8,
    request: (u8, u8),
    value: u16,
    index: u16,
    out: &mut [u8],
) -> Result<usize, i32> {
    control_no_data(xhci, slot, request, value, index)?;
    out.fill(0);
    Ok(out.len())
}

pub fn reset_bulk(_xhci: u32, _slot: u8, dir_in: bool) -> Result<(), i32> {
    DEVICE.with(|d| d.borrow_mut().resets.push(dir_in));
    Ok(())
}
