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

//! The prompt and block header agree on the shortened path.

use crate::support::line;
use crate::term::cwd::{shorten, HOME};

#[test]
fn the_prompt_shows_a_bare_tilde_in_the_home_itself() {
    assert_eq!(line(b"station", HOME, HOME), "nonos@station ~ % ls");
}

#[test]
fn the_prompt_shows_the_folder_under_the_tilde() {
    let mut cwd = Vec::from(HOME);
    cwd.extend_from_slice(b"/workspace");
    assert_eq!(line(b"station", &cwd, HOME), "nonos@station ~/workspace % ls");
}

#[test]
fn shorten_agrees_with_the_block_header() {
    let mut b = [0u8; 160];
    assert_eq!(shorten(HOME, HOME, &mut b), b"~");
    let mut c = Vec::from(HOME);
    c.extend_from_slice(b"/workspace");
    let mut b2 = [0u8; 160];
    assert_eq!(shorten(&c, HOME, &mut b2), b"~/workspace");
}

#[test]
fn shorten_leaves_a_path_outside_the_home_alone() {
    let mut b = [0u8; 160];
    assert_eq!(shorten(b"/etc/keys", HOME, &mut b), b"/etc/keys");
}
