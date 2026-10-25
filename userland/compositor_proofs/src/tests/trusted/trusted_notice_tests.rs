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

//! When the input router is told who holds the trusted band: at once on a
//! change (a failed notice tried again every RETRY_MS), again each
//! REFRESH_MS while a hold is up so a lost notice or a restarted router is
//! put right, and never while nothing is held and nothing changed.

use crate::trusted_rule::{due, REFRESH_MS, RETRY_MS};

const SHELL: u32 = 7;

#[test]
fn the_router_is_told_on_a_change_and_again_while_held() {
    assert!(!due(None, None, 0, 10_000), "nothing held, nothing told: silence");
    assert!(!due(Some(SHELL), None, 0, RETRY_MS - 1), "not before the retry gap");
    assert!(due(Some(SHELL), None, 0, RETRY_MS), "a new hold");
    assert!(!due(Some(SHELL), Some(SHELL), 0, REFRESH_MS - 1));
    assert!(due(Some(SHELL), Some(SHELL), 0, REFRESH_MS), "told again each second");
    assert!(due(None, Some(SHELL), 0, RETRY_MS), "the release");
    assert!(!due(None, Some(SHELL), 0, RETRY_MS - 1));
}
