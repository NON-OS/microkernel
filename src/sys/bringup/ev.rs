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

//! One bring-up event, and the word each is written as.

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ev {
    /// Its capsule was spawned.
    Spawned,
    /// Its capsule failed to spawn.
    Failed,
    /// Held until what it names is ready.
    Waiting(&'static str),
    /// What it waited on is ready.
    Ready(&'static str),
    /// What it waited on did not come within its bound; it went on without.
    GaveUp(&'static str),
    /// Its device was read and answered.
    Probed,
    /// What it read was published for the rest of the system.
    Published,
    /// No hardware for it, for the reason given.
    Absent(&'static str),
}

/// What a device's last event says of it, for the summary.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum State {
    Up,
    Absent,
    Waiting,
    Failed,
}

impl Ev {
    pub fn word(&self) -> &'static [u8] {
        match self {
            Ev::Spawned => b"spawned",
            Ev::Failed => b"failed",
            Ev::Waiting(_) => b"waiting",
            Ev::Ready(_) => b"ready",
            Ev::GaveUp(_) => b"gave-up",
            Ev::Probed => b"probed",
            Ev::Published => b"published",
            Ev::Absent(_) => b"absent",
        }
    }

    /// The `on=` part: what it waited on.
    pub fn on(&self) -> Option<&'static str> {
        match self {
            Ev::Waiting(w) | Ev::Ready(w) | Ev::GaveUp(w) => Some(w),
            _ => None,
        }
    }

    /// The `why=` part: why there is no hardware.
    pub fn why(&self) -> Option<&'static str> {
        match self {
            Ev::Absent(w) => Some(w),
            _ => None,
        }
    }

    pub fn state(&self) -> State {
        match self {
            Ev::Spawned | Ev::Ready(_) | Ev::Probed | Ev::Published => State::Up,
            Ev::Absent(_) => State::Absent,
            Ev::Waiting(_) => State::Waiting,
            Ev::Failed | Ev::GaveUp(_) => State::Failed,
        }
    }
}
