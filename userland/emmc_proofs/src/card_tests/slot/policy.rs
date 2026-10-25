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

//! What a removable slot does on each look at its card detect, every case.

use crate::emmc::slot::policy::{step, Step};

#[test]
fn every_case_of_card_detect() {
    for have in [false, true] {
        for inserted in [false, true] {
            for removed in [false, true] {
                for tried in [false, true] {
                    let want = match (have, inserted) {
                        (true, false) => Step::Drop,
                        (false, false) => Step::Empty,
                        (true, true) if removed => Step::Swap,
                        (true, true) => Step::Keep,
                        // A new card, or one not yet tried.
                        (false, true) if removed || !tried => Step::BringUp,
                        (false, true) => Step::Keep,
                    };
                    assert_eq!(
                        step(have, inserted, removed, tried),
                        want,
                        "{have} {inserted} {removed} {tried}"
                    );
                }
            }
        }
    }
}
