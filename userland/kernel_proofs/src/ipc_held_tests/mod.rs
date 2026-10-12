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

//! The endpoints held to their services, and the gate a send straight into a
//! process's inbox must pass, run as the kernel runs them. On the old kernel
//! MkIpcSendToPid checked nothing and every card's driver took any sender:
//! `a_send_by_pid_takes_the_network_gate` and `a_card_refuses_an_app` fail
//! there.

#[path = "../../../../src/services/registry/held.rs"]
mod held;
// held_table.rs reaches the cards as its grandparent's `nics`, as in the kernel.
#[path = "../../../../src/services/registry/nics.rs"]
mod nics;
// And the USB hosts as its grandparent's `usb_hosts`.
#[path = "../../../../src/services/registry/usb_hosts.rs"]
mod usb_hosts;

mod card_names;
mod card_readers;
mod cards;
mod ccid_tests;
mod cdc_acm_tests;
mod classified;
mod common;
mod devices;
mod gates;
mod names;
mod single_client;
mod usb_host_tests;
mod usb_mic_tests;
mod usb_spawn_tests;
mod uvc_tests;
