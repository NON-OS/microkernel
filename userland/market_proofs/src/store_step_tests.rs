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

//! What Enter and `u` do in the marketplace window for each place a listing
//! can stand: a stopped uninstall is retried as an uninstall, never as an
//! install, and an uninstall is asked of the system for a listing this boot
//! never saw installed.

use nonos_market_proto::reason::*;

use crate::store::next_step::{
    can_open, on_enter, on_remove, refused_uninstall, uninstalling, Enter, Uninstall,
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

/*
 * With no signed catalogue there is no model catalogue either, so nothing
 * installs and nothing downloads: the pane promises only what still works,
 * a model already on the disk, in lines short enough for the list pane.
 */
#[test]
fn no_catalogue_promises_only_a_model_already_here() {
    use crate::store::failure::Failure;
    use nonos_market_proto::status::E_NODATA;
    let said = String::from_utf8(Failure::Status(E_NODATA).catalogue_trouble()).unwrap();
    assert!(said.contains("ENODATA") && said.contains("nothing can be installed"), "{said}");
    assert!(said.contains("already on this disk"), "{said}");
    for line in said.lines() {
        assert!(line.len() <= 60, "{line:?} is {} long", line.len());
    }
}

/* `o` starts what can start: never a package that is not installed. */
#[test]
fn only_what_can_start_is_asked_to_open() {
    assert!(can_open(b"linux.jq", Progress::Installed));
    for p in [Progress::Idle, Progress::Removed, Progress::Failed(WHY_PACKAGE), Progress::Queued] {
        assert!(!can_open(b"linux.jq", p), "{p:?}");
    }
    // A tier's window opens, and says itself when its model is not here.
    assert!(can_open(b"linux.qwen-small", Progress::Idle));
    assert!(can_open(b"linux.qwen-qwen3-4b", Progress::Removed));
}
