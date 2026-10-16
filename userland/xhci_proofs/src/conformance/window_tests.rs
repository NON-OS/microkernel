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

//! Every register the driver reaches lies inside the window it mapped. A
//! device claiming the xHCI class can say it has 255 root ports behind a
//! BAR too small to hold their register sets; each port access then read
//! past the mapping.

use nonos_devmodel::FakeBar;
use nonos_libc::MMIO_GRANT;

use crate::constants::{CAPLENGTH, DBOFF, HCCPARAMS1, HCSPARAMS1, RTSOFF};
use crate::error::XhciError;
use crate::handles::BrokerHandles;
use crate::setup::layout::read_layout;

const CAP: u64 = 0x20;
/// Runtime and doorbell registers each with the page behind them, and room
/// for 206 port register sets past operational offset 0x400.
const WINDOW: u64 = 0x1100;

/// A controller window of `WINDOW` bytes that says it has `ports` ports.
fn layout_with(ports: u8) -> Result<u8, XhciError> {
    let bar = FakeBar::new(WINDOW as usize);
    bar.present32(CAPLENGTH as usize, CAP as u32);
    bar.present32(HCSPARAMS1 as usize, (ports as u32) << 24 | 8);
    bar.present32(HCCPARAMS1 as usize, 0x1);
    bar.present32(RTSOFF as usize, 0x60);
    bar.present32(DBOFF as usize, 0x80);
    let handles = BrokerHandles::new(1, 1, MMIO_GRANT, bar.base(), 0);
    read_layout(&handles, WINDOW).map(|l| l.max_ports)
}

#[test]
fn a_controller_whose_ports_lie_past_its_window_is_refused() {
    let fits = ((WINDOW - CAP - 0x400) / 0x10) as u8;
    assert_eq!(layout_with(2), Ok(2));
    assert_eq!(layout_with(fits), Ok(fits), "the last set ends at the window's end");
    assert_eq!(layout_with(fits + 1), Err(XhciError::ControllerUnsupported));
    assert_eq!(layout_with(255), Err(XhciError::ControllerUnsupported));
}
