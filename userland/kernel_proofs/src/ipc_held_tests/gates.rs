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

//! The gate a send straight into a process's inbox must pass.

use super::common::{owner, IPC, NETWORK};
use super::held::{callers_of, endpoint_admits, inbox_admits};

#[test]
fn an_endpoint_off_the_list_is_as_it_was() {
    for ep in
        ["net.sockets", "app.about", "vfs", "driver.nvme0", "driver.ahci0", "driver.virtio_blk0"]
    {
        assert!(callers_of(ep).is_none(), "{ep}");
        assert!(endpoint_admits(ep, owner(&[])), "{ep}");
    }
}

#[test]
fn a_send_by_pid_takes_the_network_gate() {
    // net.sockets' process serves its endpoint and its reply inbox.
    let sockets = [("net.sockets", IPC | NETWORK), ("endpoint.net.sockets.reply", IPC)];
    assert!(!inbox_admits(sockets, IPC, owner(&[])), "IPC alone wrote into net.sockets");
    assert!(inbox_admits(sockets, IPC | NETWORK, owner(&[])));
}

#[test]
fn a_send_by_pid_takes_every_gate_the_process_serves() {
    let app = [("app.about", IPC), ("endpoint.app.about.reply", IPC)];
    assert!(inbox_admits(app, IPC, owner(&[])), "an app's window lost its focus frames");
    // A process serving one endpoint that needs more is held to it.
    let mixed = [("app.about", IPC), ("net.dns", IPC | NETWORK)];
    assert!(!inbox_admits(mixed, IPC, owner(&[])));
    // A card's inbox by pid is held like the card by name.
    let card = [("driver.e1000_0", IPC), ("endpoint.driver.e1000_0.reply", IPC)];
    assert!(!inbox_admits(card, IPC | NETWORK, owner(&["app.browser"])));
    assert!(inbox_admits(card, IPC | NETWORK, owner(&["net.core"])));
    // A process that serves nothing is held to nothing beyond the syscall's own gate.
    assert!(inbox_admits([], IPC, owner(&[])));
}
