// NONOS Operating System (AGPL-3.0-or-later)
//! The scripted device's state: its answers to come and what it was sent.

use std::cell::RefCell;
use std::collections::VecDeque;

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
    /// Each bulk OUT by pair, and the pair of each bulk IN (pairs.rs).
    pub sent: Vec<(u8, Vec<u8>)>,
    pub read_on: Vec<u8>,
    /// The bytes of a CBW the device says it took, when not all 31.
    pub cbw_takes: Option<usize>,
}

thread_local! {
    pub(super) static DEVICE: RefCell<Device> = RefCell::new(Device::default());
}

/// Start over with `answers` for the bulk INs to come.
pub fn script(answers: Vec<Answer>) {
    DEVICE.with(|d| *d.borrow_mut() = Device { answers: answers.into(), ..Device::default() });
}

/// Look at the device.
pub fn device<T>(f: impl FnOnce(&Device) -> T) -> T {
    DEVICE.with(|d| f(&d.borrow()))
}

/// The bytes of each CBW the device will say it took.
pub fn cbw_takes(n: Option<usize>) {
    DEVICE.with(|d| d.borrow_mut().cbw_takes = n);
}
