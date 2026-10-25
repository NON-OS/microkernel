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

//! The event page or the request page refused: the port and the claim
//! given back, and the page too when one was made.

use nonos_libc::{script, script_dma, Script};

use super::refusals::{refused, then};
use super::util::DEVICE;

#[test]
fn a_refused_event_page_gives_the_port_back() {
    script(Script { mmio: -12, ..Script::default() });
    let tail = [("mmio_map", 1), ("pio_release", 50), ("device_release", DEVICE)];
    assert_eq!(refused(), then(3, &tail));
}

// The request port names the page in 32 bits; a page above 4 GiB would be
// read somewhere else entirely.
#[test]
fn a_refused_or_high_request_page_gives_everything_back() {
    for answer in [Err(-12), Ok((0xFFFF_F800, 0))] {
        script_dma(answer);
        let got = refused();
        let back = [("mmio_unmap", 60), ("pio_release", 50), ("device_release", DEVICE)];
        assert_eq!(got[got.len() - 3..], back);
        assert_eq!(answer.is_ok(), got.contains(&("dma_unmap", 70)));
    }
}
