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

//! A stopped or refused uninstall, and one asked for a listing this boot never installed.

use nonos_market_proto::reason::*;

use crate::store::next_step::{
    on_enter, on_remove, refused_uninstall, uninstalling, Enter, Uninstall,
};
use crate::store::progress::Progress;

#[test]
fn a_refused_uninstall_is_retried_as_an_uninstall() {
    // The button reads Retry; Enter must ask to uninstall again.
    assert_eq!(Progress::Refused.button(true), b"Retry");
    assert_eq!(on_enter(Progress::Refused, true), Enter::Remove);
    assert!(refused_uninstall(Progress::Refused, true));
    // A refused install is retried as an install.
    assert_eq!(on_enter(Progress::Refused, false), Enter::Install);
    assert!(!refused_uninstall(Progress::Refused, false));
}

#[test]
fn a_stopped_uninstall_is_known_after_the_window_is_reopened() {
    // A window opened afresh has no memory of asking; the reason says it.
    for code in [WHY_REMOVE_PARTIAL, WHY_MODEL_DOWNLOADING] {
        assert!(uninstalling(Progress::Failed(code), false), "{code}");
        assert_eq!(on_enter(Progress::Failed(code), false), Enter::Remove, "{code}");
        assert_eq!(on_remove(Progress::Failed(code)), Uninstall::Ask, "{code}");
    }
    assert_eq!(on_enter(Progress::Failed(WHY_MODEL_KEPT), false), Enter::Cannot);
    assert!(uninstalling(Progress::Removing, false));
}

#[test]
fn an_uninstall_that_found_nothing_offers_install() {
    let p = Progress::Failed(WHY_NOT_INSTALLED);
    assert!(p.uninstalled());
    assert_eq!(p.button(true), b"Install");
    assert_eq!(on_enter(p, true), Enter::Install);
    assert_eq!(on_remove(p), Uninstall::NotInstalled);
}

#[test]
fn a_listing_this_boot_never_installed_is_asked_about_all_the_same() {
    // A tier's model kept on an installed disk from an earlier boot.
    assert_eq!(on_remove(Progress::Idle), Uninstall::Ask);
    assert_eq!(on_remove(Progress::Installed), Uninstall::Ask);
    assert_eq!(on_remove(Progress::Removed), Uninstall::NotInstalled);
}
