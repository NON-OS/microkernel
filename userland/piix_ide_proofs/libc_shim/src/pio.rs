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

//! Port I/O, handed to the model the test installed: grant 50 plus the BAR.

use std::cell::RefCell;

pub trait Ports {
    fn read(&mut self, grant: u64, offset: u16, width: u8) -> u32;
    fn write(&mut self, grant: u64, offset: u16, width: u8, value: u32);
}

thread_local! {
    static PORTS: RefCell<Option<Box<dyn Ports>>> = const { RefCell::new(None) };
}

pub fn install_ports(ports: Box<dyn Ports>) {
    PORTS.with(|p| *p.borrow_mut() = Some(ports));
}

fn with<R>(f: impl FnOnce(&mut dyn Ports) -> R) -> R {
    PORTS.with(|p| f(p.borrow_mut().as_mut().expect("no ports installed on this thread").as_mut()))
}

pub fn mk_pio_read(grant: u64, offset: u16, width: u8, out: *mut u32) -> i64 {
    let v = with(|p| p.read(grant, offset, width));
    // SAFETY: the caller hands a writable u32, as the real call requires.
    unsafe { *out = v };
    0
}

pub fn mk_pio_write(grant: u64, offset: u16, width: u8, value: u32) -> i64 {
    with(|p| p.write(grant, offset, width, value));
    0
}
