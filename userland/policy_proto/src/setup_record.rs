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

//! What first-boot setup keeps when the person chose a mode that keeps state.
//!
//! Two files in the vfs store. The store overwrites a record only with one of
//! the same length, so both lengths are fixed here. Setup writes the answers
//! first and the marker last, and the policy service restores the answers only
//! beside a marker, so a half-finished save restores nothing.

pub const SETUP_DIR: &[u8] = b"/nonos/setup";
pub const ANSWERS_PATH: &[u8] = b"/nonos/setup/answers";
pub const DONE_PATH: &[u8] = b"/nonos/setup/done";

pub const ANSWERS_LEN: usize = 7;
pub const DONE: [u8; 4] = *b"NSD1";
const MAGIC: [u8; 4] = *b"NSA1";

/// The policy fields setup restores on a later boot.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Answers {
    pub keyboard_layout: u8,
    pub timezone: i8,
    pub wallpaper: u8,
}

impl Answers {
    pub fn encode(&self) -> [u8; ANSWERS_LEN] {
        let mut out = [0u8; ANSWERS_LEN];
        out[..4].copy_from_slice(&MAGIC);
        out[4] = self.keyboard_layout;
        out[5] = self.timezone as u8;
        out[6] = self.wallpaper;
        out
    }

    /// `None` for anything but a record `encode` wrote. Zeros, which is how
    /// the store withdraws a record, have no magic and so read as absent.
    pub fn decode(raw: &[u8]) -> Option<Answers> {
        if raw.len() != ANSWERS_LEN || raw[..4] != MAGIC {
            return None;
        }
        let timezone = raw[5] as i8;
        if !(-12..=14).contains(&timezone) {
            return None;
        }
        Some(Answers { keyboard_layout: raw[4], timezone, wallpaper: raw[6] })
    }
}

/// Whether `raw` is the marker setup writes once it has kept its answers.
pub fn is_done(raw: &[u8]) -> bool {
    raw == DONE
}
