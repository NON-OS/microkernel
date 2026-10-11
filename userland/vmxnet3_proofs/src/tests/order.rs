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

//! What must be in place before ACTIVATE_DEV reads it.

use super::util::up;
use crate::bench::{Bench, CONTROL_DEV};
use crate::model::{take, Event};

#[test]
fn the_address_and_the_shared_area_are_set_before_activation() {
    let mut bench = Bench::new();
    up(&mut bench);
    let log = take().log;
    let at = |want: &dyn Fn(&Event) -> bool| log.iter().position(want).expect("event");
    let mac = at(&|e| matches!(e, Event::Mac(_)));
    let shared = at(&|e| *e == Event::Shared(CONTROL_DEV));
    let activate = at(&|e| matches!(e, Event::Command { cmd: 0xCAFE_0000, .. }));
    assert!(mac < activate && shared < activate);
}
