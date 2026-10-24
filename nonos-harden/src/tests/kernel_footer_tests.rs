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

//! A kernel whose footer's sizes do not add up to the file, or that has no
//! footer, is refused before any part of it is used; and with no tree to
//! hold the kernel's keys, the check does not run.

use super::fake::Fake;
use super::kernel_tests::{fails_with, poke};
use super::sealed_fixture::Sealed;
use crate::check::Status;
use crate::release::{kernel_check, load};

#[test]
fn footer_sizes_that_do_not_add_up_are_refused() {
    for field in [16, 28, 32, 36, 40, 44] {
        let s = Sealed::dev();
        poke(&s, |len| len - 64 + field);
        fails_with(&s, "do not add up");
    }
    let s = Sealed::dev();
    poke(&s, |len| len - 64);
    fails_with(&s, "no NONOSIMG footer");
}

#[test]
fn with_no_tree_to_hold_the_keys_it_does_not_run() {
    let s = Sealed::dev();
    let mut ctx = s.ctx(Fake::default());
    ctx.tree = None;
    load::check(&mut ctx);
    assert_eq!(kernel_check::check(&mut ctx).status, Status::NotRun);
}
