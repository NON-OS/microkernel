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

//! The bring-up's stage order, each before what needs it.

use crate::bring::stage::Stage;

fn order() -> Vec<Stage> {
    let mut s = Stage::Reset;
    let mut seen = vec![s];
    while s != Stage::Up {
        s = s.next();
        seen.push(s);
    }
    seen
}

fn at(s: Stage) -> usize {
    order().iter().position(|&x| x == s).unwrap()
}

#[test]
fn the_reset_comes_first_and_up_is_reached_once() {
    let o = order();
    assert_eq!(o[0], Stage::Reset);
    assert_eq!(o.iter().filter(|&&s| s == Stage::Up).count(), 1);
    assert_eq!(o.len(), 9, "every stage visited once");
}

#[test]
fn each_stage_comes_after_what_it_needs() {
    // AUTOC is put back straight after the reset changed it.
    assert_eq!(Stage::Reset.next(), Stage::Mode);
    // Filters hold the drawn address before the receiver can run.
    assert!(at(Stage::Address) < at(Stage::Receiver));
    // The settings the rings rely on come before them.
    assert!(at(Stage::Settings) < at(Stage::TxRing));
    // The receiver goes on last, once there is a ring to receive into.
    assert!(at(Stage::RxRing) < at(Stage::Receiver));
    assert_eq!(Stage::Receiver.next(), Stage::Up);
}
