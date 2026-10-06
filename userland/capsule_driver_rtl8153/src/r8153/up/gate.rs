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

//! The RX gate (Linux rxdy_gated_en): closed while the chip is set up,
//! opened by rtl_enable as the last step before traffic.

use nonos_usbnet::Bus;

use crate::r8153::ocp::{update_word, Dev, PLA};
use crate::r8153::regs::bits::RXDY_GATED_EN;
use crate::r8153::regs::pla::MISC_1;

pub fn rxdy_gate<B: Bus>(dev: &mut Dev<B>, closed: bool) -> Result<(), i32> {
    let (clear, set) = if closed { (0, RXDY_GATED_EN) } else { (RXDY_GATED_EN, 0) };
    update_word(dev, PLA, MISC_1, clear, set)
}
