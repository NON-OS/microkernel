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
//! What the hub class needs of a USB host controller driver and of the
//! The walker's loop: the USB hosts looked up (a host that registers later
//! is looked up at every pass), then, on every host, its root ports looked
//! at for hubs and every hub's ports walked; between passes the service
//! inbox answers a health check.

use alloc::vec::Vec;

use nonos_libc::{mk_exit, say_absent, EXIT_ABSENT};
use nonos_usb_hub::{roots, scan_hubs, Hub, Parker};

use crate::host::Wire;
use crate::hosts::Hosts;
use crate::ports;
use crate::serve::answer;
use crate::usb_hosts::USB_HOSTS;

const DRIVER: &[u8] = b"driver.usb_hub";
/// The hosts register before this driver is spawned; a lookup racing a
/// spawn is tried again for about two seconds, then there is no host.
const LOOKUP_PASSES: u32 = 8;
/// A pass over every hub, then this long for the inbox.
const PASS_MS: u64 = 250;

struct Held {
    hubs: Vec<Hub>,
    tried: [u8; 256],
}

fn lookup(name: &str) -> Option<u32> {
    let (mut port, mut pid) = (0u32, 0u32);
    let rc = nonos_libc::mk_service_lookup(name.as_ptr(), name.len(), &mut port, &mut pid);
    (rc == 0).then_some(port)
}

pub fn run() -> ! {
    let mut hosts = Hosts::new(USB_HOSTS, || Held { hubs: Vec::new(), tried: [0; 256] });
    let found = (0..LOOKUP_PASSES).any(|_| hosts.find(lookup) > 0 || answer(PASS_MS));
    if !found && hosts.find(lookup) == 0 {
        say_absent(DRIVER);
        mk_exit(EXIT_ABSENT)
    }
    loop {
        hosts.find(lookup);
        hosts.each(|port, held| {
            let mut w = Wire { port };
            if let Some(root) = ports::roots(port) {
                roots(&mut w, &root, &mut held.hubs, &mut held.tried);
            }
            scan_hubs(&mut w, &mut held.hubs, &mut Parker);
        });
        answer(PASS_MS);
    }
}
