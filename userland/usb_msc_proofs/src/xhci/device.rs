// NONOS Operating System (AGPL-3.0-or-later)
//! The scripted device behind driver.xhci0, and what was asked of it.

use std::cell::RefCell;
use std::collections::VecDeque;

use super::Port;

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
    /// Each pipe reset, IN or not; how many bulk OUTs to come go a byte short.
    pub resets: Vec<bool>,
    pub short_outs: u8,
    /// The operation code of each command block sent.
    pub commands: Vec<u8>,
    /// The connected ports the controller lists.
    pub ports: Vec<Port>,
    /// The device's configuration descriptor and highest LUN, the serial
    /// number string descriptor of the device on each port that has one, and
    /// the errno its string descriptor requests fail with, if they do.
    pub config: Vec<u8>,
    pub max_lun: u8,
    pub serials: Vec<(u8, Vec<u8>)>,
    pub strings_fail: Option<i32>,
    /// Once the answers run out, each command is answered as by a unit that
    /// never comes ready (`unready`); bulk INs since the last CBW.
    pub not_ready: bool,
    pub phase: u8,
    /// When set, no transfer is answered and each call waits this long;
    /// else each takes `call_ms`, at least 1 ms.
    pub silent_ms: Option<u64>,
    pub call_ms: u64,
    /// The slots, addresses, and slot releases the controller was asked for.
    pub enabled: u32,
    pub addressed: Vec<u8>,
    pub disabled: Vec<u8>,
    /// No controller has registered; how often one was looked up.
    pub unregistered: bool,
    pub lookups: u32,
    /// Each bulk OUT by pair, and the pair of each bulk IN (`pairs`); the
    /// bytes of a CBW the device says it took, when not all 31.
    pub sent: Vec<(u8, Vec<u8>)>,
    pub read_on: Vec<u8>,
    pub cbw_takes: Option<usize>,
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

/// The bytes of each CBW the device will say it took.
pub fn cbw_takes(n: Option<usize>) {
    setup(|d| d.cbw_takes = n);
}

/// Change the device, or what the controller says of it.
pub fn setup<T>(f: impl FnOnce(&mut Device) -> T) -> T {
    DEVICE.with(|d| f(&mut d.borrow_mut()))
}
