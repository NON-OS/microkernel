// NONOS Operating System (AGPL-3.0-or-later)
//! Endpoint 0: control requests are recorded and succeed, GET MAX LUN
//! answers the scripted highest LUN, the device descriptor names the serial
//! number of the device on the port last addressed, if any, as string 3
//! (string requests fail when the test says so), and the configuration
//! descriptor is the scripted one.

use super::device::{device, setup};
use super::window::call;

const GET_DESCRIPTOR: (u8, u8) = (0x80, 0x06);
const DEVICE: u16 = 0x0100;
const LANGUAGES: u16 = 0x0300;

pub fn control_no_data(
    _xhci: u32,
    _slot: u8,
    request: (u8, u8),
    value: u16,
    index: u16,
) -> Result<(), i32> {
    call()?;
    setup(|d| d.controls.push((request.0, request.1, value, index)));
    Ok(())
}

/// As driver.xhci0 answers it: as many bytes as asked, zero past the
/// device's own.
pub fn control_in(
    xhci: u32,
    slot: u8,
    request: (u8, u8),
    value: u16,
    index: u16,
    out: &mut [u8],
) -> Result<usize, i32> {
    control_no_data(xhci, slot, request, value, index)?;
    let answer = device(|d| {
        let port = d.addressed.last();
        let serial = d.serials.iter().find(|s| Some(&s.0) == port).map(|s| s.1.clone());
        if let Some(e) = d.strings_fail.filter(|_| request == GET_DESCRIPTOR && value != DEVICE) {
            return Err(e);
        }
        Ok(match (request, value) {
            (GET_DESCRIPTOR, DEVICE) => {
                let mut desc = vec![18, 1];
                desc.resize(18, 0);
                desc[16] = if serial.is_some() { 3 } else { 0 };
                desc
            }
            (GET_DESCRIPTOR, LANGUAGES) => vec![4, 3, 0x09, 0x04],
            (GET_DESCRIPTOR, _) => serial.unwrap_or_default(),
            _ => vec![d.max_lun],
        })
    })?;
    out.fill(0);
    let n = answer.len().min(out.len());
    out[..n].copy_from_slice(&answer[..n]);
    Ok(out.len())
}

pub fn config_descriptor(_xhci: u32, _slot: u8, out: &mut [u8]) -> Result<usize, i32> {
    call()?;
    let n = device(|d| {
        let n = d.config.len().min(out.len());
        out[..n].copy_from_slice(&d.config[..n]);
        n
    });
    Ok(n)
}
