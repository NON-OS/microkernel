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

//! The host brought up over the Three-wire UART exactly as over USB: link
//! establishment first, then the same standard commands, each a reliable
//! packet. A controller that never answers SYNC, or that starts the link
//! over under the host, is a lost link, not a wait without end.

use super::h5_port::H5Port;
use crate::sim::config::{Config, ADDR};
use nonos_bt_hci::uart::h5::H5Hci;
use nonos_bt_hci::{HciError, Host, Transport};
use nonos_bt_wire::addr::BdAddr;

#[test]
fn bring_up_over_h5_reads_the_controller_as_over_usb() {
    let mut t = H5Hci::new(H5Port::new(Config::default()), 10);
    t.establish(2_000).unwrap();
    let mut host = Host::new(t);
    let c = host.bring_up().unwrap();
    assert_eq!((c.addr, c.acl_len, c.acl_total), (BdAddr(ADDR), 27, 4));
    let sent = &host.transport().port_mut().commands;
    assert_eq!(sent[0], [0x03, 0x0c, 0x00], "Reset first, no indicator: the type says it");
    assert_eq!(sent.len(), 8);
}

#[test]
fn a_controller_that_never_answers_sync_is_no_link() {
    struct Mute(u64);
    impl nonos_bt_hci::uart::UartPort for Mute {
        fn write(&mut self, _: &[u8]) -> Result<(), i32> {
            Ok(())
        }
        fn read(&mut self, _: &mut [u8]) -> Result<usize, i32> {
            Ok(0)
        }
        fn now_ms(&mut self) -> u64 {
            self.0
        }
        fn pause(&mut self) {
            self.0 += 1;
        }
    }
    assert_eq!(H5Hci::new(Mute(0), 10).establish(2_000), Err(HciError::NoLink));
}

#[test]
fn a_controller_that_starts_over_under_the_host_is_no_link() {
    let mut t = H5Hci::new(H5Port::new(Config::default()), 10);
    t.establish(2_000).unwrap();
    let port = t.port_mut();
    port.peer = Default::default();
    let sync = port.peer.poll_out(port.clock, 10).unwrap();
    port.to_host.extend(sync);
    assert_eq!(t.poll_event(), Err(HciError::NoLink));
}
