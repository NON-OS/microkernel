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

//! Setup writes the keyboard layout as the keyboard step is confirmed, so the
//! name and the Wi-Fi passphrase typed after it come through that layout.

use std::cell::RefCell;

use crate::keyboard_live::write_on_advance;

const PORT: u32 = 7;
const IT: u8 = 5;

#[test]
fn confirming_the_step_writes_the_chosen_layout_once() {
    let writes = RefCell::new(Vec::new());
    let r = write_on_advance(true, PORT, IT, |p, v| {
        writes.borrow_mut().push((p, v));
        Ok(())
    });
    assert_eq!(r, Some(Ok(())));
    assert_eq!(*writes.borrow(), vec![(PORT, IT)]);
}

#[test]
fn moving_through_the_list_or_going_back_writes_nothing() {
    let writes = RefCell::new(0);
    let r = write_on_advance(false, PORT, IT, |_, _| {
        *writes.borrow_mut() += 1;
        Ok(())
    });
    assert_eq!(r, None);
    assert_eq!(*writes.borrow(), 0);
}

#[test]
fn with_no_store_nothing_is_written() {
    let r = write_on_advance(true, 0, IT, |_, _| panic!("no store to write to"));
    assert_eq!(r, None);
}

#[test]
fn a_refusal_is_handed_back_not_hidden() {
    assert_eq!(write_on_advance(true, PORT, IT, |_, _| Err(-13)), Some(Err(-13)));
}
