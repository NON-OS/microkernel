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
//! The shell names that work on this session: screen, jobs, help, chat.

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(crate) enum Session {
    Clear,
    Echo,
    History,
    Jobs,
    Fg,
    Bg,
    Theme,
    Help,
    Which,
    Qwen,
}

pub(crate) const NAMES: &[(&[u8], Session)] = &[
    (b"clear", Session::Clear),
    (b"echo", Session::Echo),
    (b"history", Session::History),
    (b"jobs", Session::Jobs),
    (b"fg", Session::Fg),
    (b"bg", Session::Bg),
    (b"theme", Session::Theme),
    (b"profile", Session::Theme),
    (b"help", Session::Help),
    (b"commands", Session::Help),
    (b"type", Session::Which),
    (b"which", Session::Which),
    (b"qwen", Session::Qwen),
];
