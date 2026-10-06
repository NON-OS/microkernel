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

//! The MAC reset (Linux rtl8152_nic_reset, its RTL8153 branch) and the
//! buffer manager reset (rtl_reset_bmu).

use nonos_usbnet::Bus;

use crate::r8153::fail::{at, Fail};
use crate::r8153::ocp::{read_byte, wait_until, write_byte, Dev, PLA, USB};
use crate::r8153::regs::bits::CR_RST;
use crate::r8153::regs::pla::CR;
use crate::r8153::regs::usb::{BMU_RESET, BMU_RESET_EP_IN, BMU_RESET_EP_OUT};

/// Linux looks 1000 times, 100 to 400 us apart, for CR_RST to clear.
const RESET_MS: u64 = 400;

pub fn nic_reset<B: Bus>(dev: &mut Dev<B>) -> Result<(), Fail> {
    at("MAC reset refused", write_byte(dev, PLA, CR, CR_RST))?;
    let done = |d: &mut Dev<B>| Ok(read_byte(d, PLA, CR)? & CR_RST == 0);
    at("MAC reset not done", wait_until(dev, RESET_MS, 1, done))
}

/// Both endpoints' buffers held in reset, then let go, from one read.
pub fn reset_bmu<B: Bus>(dev: &mut Dev<B>) -> Result<(), Fail> {
    let both = BMU_RESET_EP_IN | BMU_RESET_EP_OUT;
    let bmu = at("BMU unread", read_byte(dev, USB, BMU_RESET))?;
    at("BMU reset refused", write_byte(dev, USB, BMU_RESET, bmu & !both))?;
    at("BMU reset refused", write_byte(dev, USB, BMU_RESET, bmu | both))
}
