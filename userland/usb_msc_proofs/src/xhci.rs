// NONOS Operating System (AGPL-3.0-or-later)
//! A Bulk-Only device standing in for driver.xhci0, with the calls and the
//! errnos `capsule_driver_usb_msc/src/xhci` gives the disk code. Each bulk
//! IN takes the next scripted answer; a bulk OUT is taken whole, and a CBW
//! among them sets the tag the device's next CSW echoes. Control requests
//! and pipe resets are recorded and succeed.

use std::cell::RefCell;
use std::collections::VecDeque;

use crate::protocol::E_IO;

pub const BULK_MAX: usize = 4096;
pub const E_PIPE: i32 = -32;

const CBW_SIGNATURE: [u8; 4] = *b"USBC";
const CSW_SIGNATURE: [u8; 4] = *b"USBS";

/// What the device does with the next bulk IN.
pub enum Answer {
    /// Sends these bytes, as many as the host asked for.
    Data(Vec<u8>),
    /// Sends a zero-length packet.
    Zlp,
    /// Sends its CSW for the last CBW.
    Csw { residue: u32, status: u8 },
    /// Stalls the pipe.
    Stall,
}

#[derive(Default)]
pub struct Device {
    pub answers: VecDeque<Answer>,
    pub tag: u32,
    /// Each control request: `(bmRequestType, bRequest, wValue, wIndex)`.
    pub controls: Vec<(u8, u8, u16, u16)>,
    /// Each pipe reset, IN or not.
    pub resets: Vec<bool>,
}

thread_local! {
    static DEVICE: RefCell<Device> = RefCell::new(Device::default());
}

/// Start over with `answers` for the bulk INs to come.
pub fn script(answers: Vec<Answer>) {
    DEVICE.with(|d| *d.borrow_mut() = Device { answers: answers.into(), ..Device::default() });
}

/// Look at the device.
pub fn device<T>(f: impl FnOnce(&Device) -> T) -> T {
    DEVICE.with(|d| f(&d.borrow()))
}

pub fn bulk_out(_xhci: u32, _slot: u8, data: &[u8]) -> Result<usize, i32> {
    if data.len() == 31 && data[0..4] == CBW_SIGNATURE {
        let tag = u32::from_le_bytes([data[4], data[5], data[6], data[7]]);
        DEVICE.with(|d| d.borrow_mut().tag = tag);
    }
    Ok(data.len())
}

/// The next answer; none left is a transfer that never completes, which
/// driver.xhci0 answers `E_IO` once it gives up.
pub fn bulk_in(_xhci: u32, _slot: u8, out: &mut [u8]) -> Result<usize, i32> {
    DEVICE.with(|d| {
        let mut d = d.borrow_mut();
        let bytes = match d.answers.pop_front() {
            Some(Answer::Data(bytes)) => bytes,
            Some(Answer::Zlp) => Vec::new(),
            Some(Answer::Csw { residue, status }) => {
                let mut csw = CSW_SIGNATURE.to_vec();
                csw.extend_from_slice(&d.tag.to_le_bytes());
                csw.extend_from_slice(&residue.to_le_bytes());
                csw.push(status);
                csw
            }
            Some(Answer::Stall) => return Err(E_PIPE),
            None => return Err(E_IO),
        };
        let n = bytes.len().min(out.len());
        out[..n].copy_from_slice(&bytes[..n]);
        Ok(n)
    })
}

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
