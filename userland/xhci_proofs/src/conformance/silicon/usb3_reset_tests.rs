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

//! USB 3 ports that enable themselves, links that need a warm reset, and a
//! connect that drops while it is debounced.

use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::Arc;

use nonos_devmodel::{run, FakeBar};

use super::super::model::{controller, PORT1};
use super::super::port_device::reset_port1;
use super::super::port_settle::settles_enabled;
use crate::constants::{PORTSC_CCS, PORTSC_PED, PORTSC_PP, PORTSC_PR, PORTSC_PRC, PORTSC_WPR};

/// A USB 3 device on port 1 whose link trained: connected, enabled, in U0.
/// Records every write the driver makes to the port.
fn trained_usb3(writes: Arc<AtomicU32>) -> impl Fn(&FakeBar) {
    let trained = PORTSC_PP | PORTSC_CCS | PORTSC_PED;
    // Acts only on a value it did not present itself: the driver's write.
    let shown = AtomicU32::new(trained);
    move |bar| {
        let written = bar.wrote32(PORT1);
        if written != shown.load(Ordering::Relaxed) {
            writes.fetch_or(written, Ordering::Relaxed);
            // A hot reset on a trained link ends enabled, with PRC to say so.
            let next = if written & PORTSC_PR != 0 { trained | PORTSC_PRC } else { trained };
            shown.store(next, Ordering::Relaxed);
            bar.present32(PORT1, next);
        }
    }
}

#[test]
fn an_enabled_usb3_port_is_reset_before_addressing_and_never_disabled() {
    // Another class driver may have addressed the device on a trained link and
    // let it go; real devices refuse a second Address Device without a reset.
    let bar = controller(0);
    bar.present32(PORT1, PORTSC_PP | PORTSC_CCS | PORTSC_PED);
    let writes = Arc::new(AtomicU32::new(0));
    let _dev = run(&bar, trained_usb3(writes.clone()));
    reset_port1(&bar, true).expect("the port is enabled");
    // What the port shows, not the read after the driver's acknowledgement.
    assert!(settles_enabled(&bar), "the port shows enabled with its PRC acknowledged");
    let w = writes.load(Ordering::Relaxed);
    assert_ne!(w & PORTSC_PR, 0, "a hot reset was written");
    assert_eq!(w & PORTSC_WPR, 0, "no warm reset on a trained link");
    assert_eq!(w & PORTSC_PED, 0, "PED was never written as one");
}
