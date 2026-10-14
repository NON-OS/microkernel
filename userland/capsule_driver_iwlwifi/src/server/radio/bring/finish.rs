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

//! The post-ALIVE commands run, and the radio's channels, antennas and join
//! layout taken from them.

use crate::firmware::gen3::dev::Dev;
use crate::firmware::gen3::outcome::Failure;
use crate::firmware::gen3::select::Transport;
use crate::firmware::gen3::start::{rf_killed, stop_device};
use crate::firmware::gen3::ucode::Ucode;
use crate::firmware::gen3::up::{up, SCAN_IF_ADDR};
use crate::regs::Regs;

use super::super::clock::Uptime;
use super::super::grant::Grant;
use super::super::say::Line;
use super::report::join_layout;
use super::seen::Up;

pub(super) fn finish(
    mut dev: Dev<'static, Regs, Grant>,
    ucode: &Ucode<'_>,
    transport: &Transport,
    addr: Option<[u8; 6]>,
) -> Result<Up, Failure> {
    let regs = dev.m;
    let mut c = Uptime;
    let info = match up(&mut dev, &mut c, ucode, transport, addr.unwrap_or(SCAN_IF_ADDR)) {
        Ok(i) => i,
        Err(e) => {
            stop_device(regs, &mut c);
            return Err(Failure::Up(e));
        }
    };
    let mut l = Line::new();
    l.text(b"up: ")
        .num(info.nvm.channels.len() as u32)
        .text(b" channels, tx ant ")
        .num(u32::from(info.nvm.valid_tx_ant));
    if let Some(mcc) = info.mcc {
        l.text(b", regulatory ").hex(u32::from(mcc));
    }
    l.send();
    if rf_killed(regs) {
        Line::new().text(b"the hardware RF-kill switch has the radio off").send();
    }
    let join_api = join_layout(ucode);
    let (tx_ant, rx_ant) = (
        ucode.valid_tx_ant() & info.nvm.valid_tx_ant,
        ucode.valid_rx_ant() & info.nvm.valid_rx_ant,
    );
    Ok(Up { dev, channels: info.nvm.channels, addr, tx_ant, rx_ant, join_api })
}
