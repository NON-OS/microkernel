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

#![no_std]
#![no_main]

extern crate alloc;

mod r8153;

use nonos_libc::{heap_init, mk_exit};
use nonos_usbnet::say::say;
use nonos_usbnet::xhci::XhciBus;
use nonos_usbnet::{Bind, Found};

use r8153::Rtl8153;

const TAG: &[u8] = b"rtl8153";

/// # Safety
/// The capsule entry point. The kernel loader calls this once on a fresh stack
/// with the capsule's heap region reserved; it must never be called from Rust.
#[no_mangle]
pub unsafe extern "C" fn _start() -> ! {
    if heap_init().is_err() {
        mk_exit(1);
    }
    nonos_usbnet::run(TAG, bind);
}

/// The driver's binding, with the chip it found and its link changes put
/// on the log; the proofs run the binding without a log.
fn bind(bus: XhciBus, found: &Found) -> Bind<Rtl8153<XhciBus>> {
    let mut out = r8153::bind(bus, found);
    if let Bind::Ours(nic) = &mut out {
        nic.note = note;
        say(TAG, &[b"chip ", nic.version().name()]);
    }
    out
}

fn note(parts: &[&[u8]]) {
    say(TAG, parts);
}
