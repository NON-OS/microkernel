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

//! The store's Open of a program made for a terminal reaches the Terminal:
//! the run stays app.linux.run, the one the desktop shell takes the ask
//! from, rather than starting again in the network role first; it names
//! the package as its listing does (`kali.<package>`), a word the shell
//! takes, where `deb:` has a `:` it refuses; and the Terminal's `linux`
//! command runs an installed package by that word in its family's tree.

use nonos_linux_policy::mask::{FILES_HOME, NET_ANON, TTY, WAYLAND};

use crate::family::{listed, run_name};

#[path = "../../../../capsule_linux/src/linux/terminal_want.rs"]
mod terminal_want;

const RUN_NET: &str = include_str!("../../../../capsule_linux/src/linux/run_net.rs");
const HAND: &str = include_str!("../../../../capsule_linux/src/linux/terminal_hand.rs");
const LAUNCH: &str = include_str!("../../../../capsule_linux/src/linux/terminal_launch/launch.rs");

#[test]
fn a_run_is_named_as_its_listing_names_it() {
    assert_eq!(listed("deb:nonos-simplex-chat"), "kali.nonos-simplex-chat");
    assert_eq!(listed("pacman:nmap"), "blackarch.nmap");
    assert_eq!(listed("helix"), "helix");
}

#[test]
fn the_listing_word_names_the_run_again() {
    for name in
        ["deb:nonos-simplex-chat", "pacman:nmap", "helix", "deb:nonos-darkfi-drk", "py3-six"]
    {
        assert_eq!(run_name(&listed(name)).as_deref(), Some(name), "{name}");
    }
}

#[test]
fn only_a_package_name_is_a_word() {
    let too_long = "a".repeat(65);
    for word in [
        "",
        "deb:jq",
        "/usr/bin/jq",
        "../jq",
        "-x",
        "Jq",
        "jq bar",
        "kali.",
        "blackarch.",
        &too_long,
    ] {
        assert_eq!(run_name(word), None, "{word:?}");
    }
    assert_eq!(run_name(&"a".repeat(64)).as_deref(), Some("a".repeat(64).as_str()));
}

#[test]
fn a_word_in_no_namespace_is_alpine_s() {
    for word in ["kali", "kalix.jq", "black.arch", "nonos-simplex-chat"] {
        assert_eq!(run_name(word).as_deref(), Some(word), "{word}");
    }
}

#[test]
fn only_a_window_run_of_a_terminal_program_is_for_the_terminal() {
    use terminal_want::for_terminal;
    let tty = TTY | FILES_HOME | NET_ANON;
    assert!(for_terminal(true, tty, false));
    assert!(!for_terminal(false, tty, false), "a cli run is on its terminal already");
    assert!(!for_terminal(true, tty, true), "the Terminal started it");
    assert!(!for_terminal(true, tty | WAYLAND, false), "a window program");
    assert!(!for_terminal(true, FILES_HOME | NET_ANON, false), "no terminal in its box");
}

#[test]
fn early_leaves_a_run_for_the_terminal_in_its_role() {
    let body = &RUN_NET[RUN_NET.find("pub(super) fn early(").expect("early is gone")..];
    let body = &body[..body.find("\n}\n").expect("early ends")];
    let kept = body.find("start_guest::keep(path, bytes, app)").expect("not kept");
    let wants = body
        .find("if super::terminal_hand::wants(app.mask) {\n        return;\n    }")
        .expect("no stay");
    let asked = body.find("net_step(app.mask").expect("no net step");
    assert!(kept < wants && wants < asked, "decided after the proof and before the role");
}

#[test]
fn the_hand_over_names_the_listing_word_and_the_check_is_the_mask() {
    assert!(HAND.contains("listed(&name).as_bytes()"), "the run's name is not handed as is");
    assert!(!HAND.contains("name.as_bytes(), &mut rx"));
    assert!(HAND.contains("if !wants(guest.app.mask) {"));
    assert!(HAND.contains("for_terminal(window, mask, super::terminal::started())"));
}

#[test]
fn a_bare_name_not_on_the_path_is_looked_for_as_an_installed_package() {
    let store = LAUNCH.find("store_read(&key(&path), max)").expect("the PATH is read first");
    let installed =
        LAUNCH.find("super::installed::installed(program, args, max)").expect("no package lookup");
    let unread = LAUNCH.find("unread(program, args").expect("no refusal");
    assert!(store < installed && installed < unread);
    assert!(LAUNCH.contains("let args = match program.contains(&b'/') {\n        true => args,"));
}

#[test]
fn a_bare_word_an_app_image_is_named_by_runs_its_program_before_the_path() {
    let image = LAUNCH.find("crate::linux::install::image_word(program)").expect("no image lookup");
    let looked = LAUNCH[image..].find("super::installed::installed(word.as_bytes(), args, max)");
    assert!(looked.is_some(), "the image is looked for as an installed package");
    let path = LAUNCH.find("terminal_path::find(program").expect("no PATH lookup");
    assert!(LAUNCH.find("wait_settled()").expect("settled") < image && image < path);
}
