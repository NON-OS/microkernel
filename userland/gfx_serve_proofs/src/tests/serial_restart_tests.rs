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
//! A serial carries its driver's pid in the high half, so a driver started
//! again under a new pid refuses what the old instance handed out, though
//! both counters start alike. A driver denied its own pid stamped 0 here
//! (the gate on b0f7e93b7), and two instances' serials then matched.

use crate::model::wire::{ask, u32s, COMPOSITOR};
use crate::model::{Pool, Vram};
use crate::server::Server;
use nonos_gfx_wire::errno::ESTALE;
use nonos_gfx_wire::{ops, Surface};

fn started(pid: u32) -> (Server<Vram, Pool>, Surface) {
    let mut s = Server::new(Vram::new(true, true), Pool::new(4096));
    s.platform.pid = pid;
    let sf = Surface::decode(&ask(&mut s, COMPOSITOR, ops::SURFACE, 0, &[]).1).unwrap();
    (s, sf)
}

#[test]
fn a_restarted_driver_refuses_the_old_instance_s_serials() {
    let (_, old) = started(0x42);
    let (mut new, now) = started(0x43);
    assert_ne!(old.serial, now.serial, "same counter, another pid");
    let flush = |serial| u32s(&[serial, 1, 0, 0, 1, 1]);
    assert_eq!(ask(&mut new, COMPOSITOR, ops::FLUSH, 0, &flush(old.serial)).0, ESTALE);
    assert_eq!(ask(&mut new, COMPOSITOR, ops::FLUSH, 0, &flush(now.serial)).0, 0);
}
