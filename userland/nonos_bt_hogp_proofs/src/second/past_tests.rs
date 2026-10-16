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
//! HID services past the bound: the first four are read and the rest left
//! unread, as Battery Services past theirs are; the device is not refused.

use super::devices::{boot_keyboard, handles};
use crate::sim::{Db, Run};
use nonos_bt_hogp::host::{Outcome, MAX_SERVICES};

fn keyboards(n: usize) -> Db {
    (0..n).fold(Db::default(), |db, _| boot_keyboard(db))
}

#[test]
fn services_past_the_bound_are_left_unread_and_the_rest_read() {
    let db = keyboards(MAX_SERVICES + 1 + 2);
    let all = handles(&db, 0x2a22);
    let mut run = Run::new(db);
    assert_eq!(run.go(), Some(Outcome::Ready { keyboard: true, mouse: false }));
    let kept: Vec<u16> = run.host.sources().iter().map(|s| s.value).collect();
    assert_eq!(kept, all[..MAX_SERVICES], "the first four");
    assert!(run.host.services_past(), "and the host says some were left");
    let mut run = Run::new(keyboards(MAX_SERVICES));
    assert_eq!(run.go(), Some(Outcome::Ready { keyboard: true, mouse: false }));
    assert!(!run.host.services_past(), "none past the bound");
}
