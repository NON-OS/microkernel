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

//! The keys the window answers, already read from the input event, where
//! they go, and how far the window is with the core.

/// Where the keys go.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Focus {
    List,
    Compose,
    Search,
    /// The person's name for the core's first start (start.rs), and that
    /// name given, waiting for the core.
    Name,
    Named,
}

/// How far the window is with the core.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Conn {
    /// No channel to the core.
    Offline,
    /// The channel is up and the chat list is asked for.
    Waiting,
    /// The chat list came.
    Ready,
}

/// A key the window answers.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Key {
    Up,
    Down,
    Tab,
    Enter,
    Esc,
    Backspace,
    /// Ctrl+K.
    Search,
    Char(char),
}
