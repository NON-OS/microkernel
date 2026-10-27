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

//! A `Transport` that replays a recorded session of the chat API, so the
//! window runs with real-shaped data before the channel to the core exists.

use alloc::collections::VecDeque;
use alloc::format;
use alloc::string::String;

use super::script::{script, Script};

/// The recorded session the window replays.
pub const SESSION: &str = include_str!("session.txt");
/// Ticks between two pushed events.
pub const EVENT_EVERY: u32 = 16;
/// The first id the transcript gives an item it makes up.
const FIRST_ID: i64 = 1000;
pub(super) const NOT_RECORDED: &str =
    "{\"type\":\"chatCmdError\",\"chatError\":{\"type\":\"error\",\
    \"errorType\":{\"type\":\"commandError\",\"message\":\"not in the transcript\"}}}";

pub struct Transcript {
    pub(super) script: Script,
    pub(super) inbox: VecDeque<String>,
    /// Events still to push: the recorded ones, then those sends made.
    pub(super) later: VecDeque<String>,
    pub(super) ticks: u32,
    /// The chat list was asked for, so events may follow.
    pub(super) started: bool,
    pub(super) next_id: i64,
}

pub(super) fn frame(corr: Option<&str>, resp: &str) -> String {
    match corr {
        Some(c) => format!("{{\"corrId\":\"{c}\",\"resp\":{resp}}}"),
        None => format!("{{\"resp\":{resp}}}"),
    }
}

impl Transcript {
    pub fn new(text: &'static str) -> Self {
        let script = script(text);
        let later = script.events.iter().map(|e| String::from(*e)).collect();
        Transcript {
            script,
            inbox: VecDeque::new(),
            later,
            ticks: 0,
            started: false,
            next_id: FIRST_ID,
        }
    }
}
