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

//! Nothing is asked while a listing moves; a stopped install is retried as one.

use nonos_market_proto::reason::*;

use crate::store::next_step::{on_enter, on_remove, Enter, Uninstall};
use crate::store::progress::Progress;

#[test]
fn nothing_is_asked_while_an_install_or_uninstall_moves() {
    for p in [Progress::Queued, Progress::Installing, Progress::Removing] {
        assert_eq!(on_enter(p, false), Enter::Wait, "{p:?}");
        assert_eq!(on_remove(p), Uninstall::Wait, "{p:?}");
    }
    assert_eq!(on_enter(Progress::Failed(WHY_NO_VOLUME), false), Enter::Wait);
}

#[test]
fn an_install_that_stopped_is_retried_as_an_install_or_not_at_all() {
    assert_eq!(on_enter(Progress::Installed, false), Enter::Open);
    assert_eq!(on_enter(Progress::Idle, false), Enter::Install);
    assert_eq!(on_enter(Progress::Removed, true), Enter::Install);
    assert_eq!(on_enter(Progress::Failed(WHY_MODEL_FETCH), false), Enter::Install);
    assert_eq!(on_enter(Progress::Failed(WHY_MODEL_TOO_LARGE), false), Enter::Cannot);
    // What a stopped download left behind can be taken away.
    assert_eq!(on_remove(Progress::Failed(WHY_MODEL_FETCH)), Uninstall::Ask);
    assert_eq!(on_remove(Progress::Refused), Uninstall::Ask);
}
