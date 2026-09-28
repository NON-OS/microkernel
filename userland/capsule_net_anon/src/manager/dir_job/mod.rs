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

//! One directory fetch at a time, advanced a step per idle turn.
//!
//! The serve loop answers callers only between turns, so a fetch that waited
//! inside a turn stopped every caller for as long as it waited: up to eight
//! seconds for a connection, ten to send and thirty to read, and a sweep of
//! seven authorities did that seven times in one turn.

mod finish;
mod job;
mod opening;
mod poll;
mod reading;
mod sending;
mod start;
mod turn;

pub use job::DirWork;
pub(super) use turn::{turn, Turn};
