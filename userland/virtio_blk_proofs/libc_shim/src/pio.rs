// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! Port I/O, answered by the device model the test attaches: every read
//! and write reaches it as the event it is, so a model can hold a reset
//! unfinished for some reads, which memory that reads back what was
//! written cannot. With no model attached, port I/O refuses to be called.

use std::cell::RefCell;

/// A device behind the port grant, offsets from its first port.
pub trait Ports {
    fn read(&mut self, port: u16, width: u8) -> u32;
    fn write(&mut self, port: u16, width: u8, value: u32);
}

thread_local! {
    static PORTS: RefCell<Option<Box<dyn Ports>>> = const { RefCell::new(None) };
}

pub fn attach_ports(device: impl Ports + 'static) {
    PORTS.with(|p| *p.borrow_mut() = Some(Box::new(device)));
}

fn with<T>(f: impl FnOnce(&mut dyn Ports) -> T) -> T {
    PORTS.with(|p| match p.borrow_mut().as_mut() {
        Some(device) => f(device.as_mut()),
        None => panic!("virtio_blk_proofs: port I/O with no device attached"),
    })
}

pub fn mk_pio_read(_grant: u64, port: u16, width: u8, out: *mut u32) -> i64 {
    let value = with(|d| d.read(port, width));
    // SAFETY: the driver passes the address of one of its own u32s.
    unsafe { *out = value };
    0
}

pub fn mk_pio_write(_grant: u64, port: u16, width: u8, value: u32) -> i64 {
    with(|d| d.write(port, width, value));
    0
}
