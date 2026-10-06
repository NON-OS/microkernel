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

//! Linux r8153_first_init, the body of rtl8153_up and rtl8153b_up: RX held
//! at the gate, the wake offload off, nothing accepted, the MAC and the
//! buffer manager reset, out of the out-of-band mode with the link list
//! rebuilt, then the frame size and FIFO settings.

use nonos_usbnet::Bus;

use super::fifo::fifo;
use super::gate::rxdy_gate;
use super::link_list::leave_oob;
use super::reset::{nic_reset, reset_bmu};
use super::teredo::teredo_off;
use crate::r8153::fail::{at, Fail};
use crate::r8153::ocp::{update_dword, Dev, PLA};
use crate::r8153::regs::bits::RCR_ACPT_ALL;
use crate::r8153::regs::pla::RCR;
use crate::r8153::Version;

pub fn first_init<B: Bus>(dev: &mut Dev<B>, v: Version) -> Result<(), Fail> {
    at("RX gate not closed", rxdy_gate(dev, true))?;
    teredo_off(dev, v)?;
    at("RX filter not cleared", update_dword(dev, PLA, RCR, RCR_ACPT_ALL, 0))?;
    nic_reset(dev)?;
    reset_bmu(dev)?;
    leave_oob(dev)?;
    fifo(dev)
}
