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

//! Per-version hardware steps copied from Linux r8169_main.c, each named
//! after the Linux function it follows, with every wait on the clock.

mod conds;
mod cplus;
pub mod eri;
mod fifo_empty;
mod init_8125;
mod init_8168g;
mod init_shared;
mod initialize;
pub mod ocp;
pub mod ocp_8125;
mod quiesce;
pub mod regs;
mod rxcfg;
mod rxdvgate;
mod start;
mod start_8125;
mod start_8168;
mod start_8168g;
mod start_8169;
mod txcfg;
mod wait;

pub use conds::request_stop;
pub use cplus::cplus_cmd;
pub use fifo_empty::wait_txrx_fifo_empty;
pub use init_8125::init_8125;
pub use init_8168g::init_8168g;
pub use initialize::initialize;
pub use quiesce::quiesce;
pub use rxcfg::rx_config;
pub use rxdvgate::{disable_rxdvgate, enable_rxdvgate};
pub use start::start;
pub use start_8125::start_8125;
pub use start_8168::start_8168;
pub use start_8168g::start_8168g;
pub use start_8169::start_8169;
pub use txcfg::tx_config;
pub use wait::{hold_ms, wait_for};
