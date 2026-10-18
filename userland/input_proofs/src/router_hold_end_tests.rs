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

//! A trusted hold ends on its own when the compositor stops telling it (the
//! lease runs out, and focus routes again), and at once when its holder
//! dies, so no prompt that is gone can leave every window deaf.

use nonos_libc::{mk_uptime_ms, INPUT_KIND_KEY_DOWN};

use crate::route::trusted_hold::{told, HOLD_LEASE_MS};
use crate::router_desk::*;
use crate::router_tests::router_trusted_tests::{held_desk, A, KEY_X};

#[test]
fn the_hold_ends_when_it_lapses_or_its_holder_dies() {
    let mut ctx = held_desk();
    nonos_libc::advance_ms(HOLD_LEASE_MS);
    key_down(&mut ctx, KEY_X, 0);
    assert_eq!(of_kind(&sent(), INPUT_KIND_KEY_DOWN), [(A, KEY_X)], "lapsed: focus again");
    ctx.trusted = told(77, mk_uptime_ms());
    nonos_libc::kill(77);
    ctx.purge_dead();
    assert_eq!(ctx.trusted, None);
}
