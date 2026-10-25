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

//! Under the no-IOMMU posture: what a grant and an interrupt may be, and the
//! words a claim says.

use super::open::{interrupts_confined, physical, unconfined_words};

#[test]
fn under_the_posture_grants_are_physical_and_interrupts_unremapped() {
    assert!(physical(false, true) && physical(true, false));
    assert!(!physical(false, false));
    assert!(interrupts_confined(false, false, true, true), "posture, production");
    assert!(!interrupts_confined(false, false, true, false));
    assert!(!interrupts_confined(false, true, true, false), "opt-in refused in production");
}

#[test]
fn an_unconfined_claim_says_plainly_that_nothing_confines_it() {
    let words = std::str::from_utf8(unconfined_words()).unwrap();
    assert!(words.contains("reaches all memory"), "{words}");
    let dev = cfg!(feature = "nonos-dev-unconfined-dma")
        && !cfg!(any(feature = "nonos-production", feature = "nonos-release"));
    assert_eq!(words.contains("development opt-in"), dev);
    assert_eq!(words.contains("not confined by hardware"), !dev);
}
