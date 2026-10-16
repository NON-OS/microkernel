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

//! A pointer with absolute axes, as a tablet has: this host decodes only
//! relative ones, so the service keeps its boot mouse.

use super::mice::absolute;
use super::pointer_mode_tests::kinds;
use crate::sim::pointer_devices::*;
use crate::sim::Run;
use nonos_bt_hogp::host::{Kind, Outcome};

#[test]
fn a_tablet_alone_stays_in_boot_mode() {
    let mut run = Run::new(mouse_alone(&absolute(), &[3, 1]));
    assert_eq!(run.go(), Some(Outcome::Ready { keyboard: false, mouse: true }));
    assert_eq!(kinds(&run), [(Kind::Mouse, MOUSE_BOOT)]);
    assert_eq!(run.peer.writes[0], (MOUSE_MODE, vec![0], true));
}
