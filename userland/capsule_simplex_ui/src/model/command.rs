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

//! The few commands a person types on the compose line.

use alloc::string::String;

use crate::proto::cmd::Cmd;

/// What a submitted line asks for, when it is a command.
pub fn command(line: &str) -> Option<Cmd> {
    let (word, rest) = line.split_once(' ').unwrap_or((line, ""));
    let rest = rest.trim();
    match word {
        "/connect" if !rest.is_empty() => Some(Cmd::Connect(String::from(rest))),
        "/invite" => Some(Cmd::Invite),
        "/address" => Some(Cmd::MakeAddress),
        _ => None,
    }
}
