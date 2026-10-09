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

//! RX aggregation off. r8153_init and r8153b_init clear RX_AGG_DISABLE so
//! the chip packs frames into one transfer of up to rx_buf_sz (16 or
//! 32 KiB). A bulk IN through driver.xhci0 moves at most BULK_MAX (4096)
//! bytes, so the chip is told to send each frame in a transfer of its
//! own, as r8153_init does for the Dell TB16 dock (dell_tb_rx_agg_bug):
//! one rx_desc and one frame of at most RMS bytes, 1546 in all.
//! RX_ZERO_EN is cleared as Linux clears it. The RX early size and
//! timeout (r8153_set_rx_early_size and _timeout) tune aggregation only,
//! and are left at the chip's values.

use nonos_usbnet::Bus;

use crate::r8153::fail::{at, Fail};
use crate::r8153::ocp::{update_word, Dev, USB};
use crate::r8153::regs::usb::{RX_AGG_DISABLE, RX_ZERO_EN, USB_CTRL};

pub fn rx_one_frame_per_transfer<B: Bus>(dev: &mut Dev<B>) -> Result<(), Fail> {
    let r = update_word(dev, USB, USB_CTRL, RX_AGG_DISABLE | RX_ZERO_EN, RX_AGG_DISABLE);
    at("RX aggregation not disabled", r)
}
